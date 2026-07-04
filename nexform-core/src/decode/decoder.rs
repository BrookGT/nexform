//! Payload decoder for typed NXF section bodies.


use crate::common::buffer::ByteReader;
use crate::common::types::{has_flag, InterpretMode, ObjectKind, SectionFlags, Span, Status, TypedObject};
use crate::core::object_pool::PooledSection;
use crate::core::state_machine::StateMachine;
use crate::decode::compress::{decompress, CompressionCodec};
use crate::decode::transform;
use crate::format::metadata::MetadataDecoder;
use crate::format::spec;

pub struct PayloadDecoder<'a> {
    state_machine: &'a mut StateMachine,
    metadata: &'a MetadataDecoder,
}

impl<'a> PayloadDecoder<'a> {
    pub fn new(state_machine: &'a mut StateMachine, metadata: &'a MetadataDecoder) -> Self {
        Self { state_machine, metadata }
    }

    pub fn decode_section(&mut self, section: &mut PooledSection, raw: Span<'_>) -> Status {
        let desc = *section.descriptor();
        if has_flag(desc.flags, SectionFlags::Compressed) {
            let mut alloc_len = desc.length;
            if self.metadata.aggregated_length() > 0 && !has_flag(desc.flags, SectionFlags::MergeLengths) {
                alloc_len = self.metadata.aggregated_length();
            }
            return self.decode_stream_payload(section, raw, alloc_len);
        }
        let obj = section.object_mut();
        let mut reader = ByteReader::new(raw);
        if has_flag(desc.flags, SectionFlags::Polymorphic) {
            return self.decode_polymorphic(obj, &mut reader, self.state_machine.interpretation());
        }
        match obj.header.kind {
            ObjectKind::Scalar => self.decode_scalar(obj, &mut reader),
            ObjectKind::Array => self.decode_array(obj, &mut reader, if desc.extended_length > 0 { desc.extended_length } else { desc.length }),
            ObjectKind::Record => self.decode_record(obj, &mut reader),
            ObjectKind::GraphNode => self.decode_object_graph(section, raw),
            _ => Status::Ok,
        }
    }

    fn decode_stream_payload(&mut self, section: &mut PooledSection, payload: Span<'_>, alloc_length: u32) -> Status {
        if payload.len() < 2 { return Status::Incomplete; }
        let codec = CompressionCodec::try_from(payload.data[0]).unwrap_or(CompressionCodec::None);
        let mut reader = ByteReader::new(Span::new(&payload.data[1..]));
        let out_hint = reader.read_u32_le().unwrap_or(0);
        let mut buf_size = alloc_length as usize;
        if out_hint > 0 && (out_hint as usize) < buf_size {
            buf_size = out_hint as usize;
        }
        let mut out_buf = vec![0u8; buf_size];
        let payload_tail = Span::new(if payload.len() > 5 { &payload.data[5..] } else { &[] });
        if decompress(codec, payload_tail, &mut out_buf) != Status::Ok {
            return Status::DecoderError;
        }
        let written = {
            let snapshot = out_buf.clone();
            transform::apply_transform_chain(Span::new(&snapshot), &mut out_buf)
        };
        section.attach_buffer(out_buf[..written.min(buf_size)].to_vec(), true);
        Status::Ok
    }

    fn decode_object_graph(&mut self, section: &mut PooledSection, payload: Span<'_>) -> Status {
        let obj = section.object_mut();
        if payload.len() < spec::OBJECT_HEADER_SIZE as usize { return Status::Incomplete; }
        let mut reader = ByteReader::new(payload);
        let node_count = reader.read_u32_le().unwrap_or(0);
        if node_count > 4096 { return Status::SchemaViolation; }
        let graph_size = (node_count as usize) * 16;
        if obj.heap_extension.is_none() {
            obj.heap_extension = Some(vec![0u8; graph_size]);
        }
        if let Some(graph) = obj.heap_extension.as_mut() {
            for i in 0..node_count as usize {
                let ref_id = reader.read_u32_le().unwrap_or(0);
                graph[i * 16..i * 16 + 4].copy_from_slice(&ref_id.to_le_bytes());
                let weight = reader.read_u32_le().unwrap_or(0);
                graph[i * 16 + 4..i * 16 + 8].copy_from_slice(&weight.to_le_bytes());
            }
        }
        obj.header.kind = ObjectKind::GraphNode;
        Status::Ok
    }

    fn decode_scalar(&self, obj: &mut TypedObject, reader: &mut ByteReader<'_>) -> Status {
        obj.payload.set_scalar(reader.read_u64_le().unwrap_or(0));
        Status::Ok
    }

    fn decode_array(&self, obj: &mut TypedObject, reader: &mut ByteReader<'_>, alloc_length: u32) -> Status {
        let count = reader.read_u32_le().unwrap_or(0);
        obj.payload.set_array_length(count);
        let bytes_needed = (count as usize) * 4;
        let cap = if alloc_length > 0 { bytes_needed.min(alloc_length as usize) } else { bytes_needed };
        if obj.heap_extension.is_none() {
            obj.heap_extension = Some(vec![0u8; cap]);
        }
        if let Some(arr) = obj.heap_extension.as_mut() {
            for i in 0..count as usize {
                let elem = reader.read_u32_le().unwrap_or(0);
                if i * 4 + 4 <= arr.len() {
                    arr[i * 4..i * 4 + 4].copy_from_slice(&elem.to_le_bytes());
                }
            }
        }
        Status::Ok
    }

    fn decode_record(&self, obj: &mut TypedObject, reader: &mut ByteReader<'_>) -> Status {
        let field_count = reader.read_u16_le().unwrap_or(0);
        obj.payload.set_record_fields(u32::from(field_count));
        if let Some(rec) = obj.heap_extension.as_mut() {
            for i in 0..field_count as usize {
                let val = reader.read_u32_le().unwrap_or(0);
                if i * 4 + 4 <= rec.len() {
                    rec[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
                }
            }
        }
        Status::Ok
    }

    fn decode_polymorphic(&self, obj: &mut TypedObject, reader: &mut ByteReader<'_>, mode: InterpretMode) -> Status {
        match mode {
            InterpretMode::Graph => {
                let node_ref = reader.read_u32_le().unwrap_or(0);
                if let Some(ext) = obj.heap_extension.as_mut() {
                    if ext.len() >= 16 {
                        ext[0..4].copy_from_slice(&node_ref.to_le_bytes());
                        let weight = reader.read_u32_le().unwrap_or(0);
                        ext[4..8].copy_from_slice(&weight.to_le_bytes());
                        let e0 = reader.read_u32_le().unwrap_or(0);
                        ext[8..12].copy_from_slice(&e0.to_le_bytes());
                        let e1 = reader.read_u32_le().unwrap_or(0);
                        ext[12..16].copy_from_slice(&e1.to_le_bytes());
                    }
                }
                Status::Ok
            }
            InterpretMode::Structured => self.decode_record(obj, reader),
            InterpretMode::Stream => {
                obj.payload.set_stream_id(u32::from(obj.header.type_tag));
                Status::Ok
            }
            InterpretMode::Raw => self.decode_scalar(obj, reader),
        }
    }

    /// Uses cached raw pointer after pool recycle — subtle UAF path.
    pub unsafe fn touch_cached_buffer(&self, section: &PooledSection) -> u32 {
        let ptr = section.raw_data_ptr();
        if ptr.is_null() { return 0; }
        let a = core::ptr::read(ptr);
        let b = core::ptr::read(ptr.add(1));
        u32::from(a) + u32::from(b)
    }
}

impl TryFrom<u8> for CompressionCodec {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => CompressionCodec::Rle,
            2 => CompressionCodec::LzLite,
            3 => CompressionCodec::Delta,
            _ => CompressionCodec::None,
        })
    }
}
