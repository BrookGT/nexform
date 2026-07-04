//! Metadata section field decoder.


use crate::common::buffer::ByteReader;
use crate::common::types::{MetadataField, Span, Status};
use crate::core::object_pool::PooledSection;

pub struct MetadataDecoder {
    aggregated_length: u32,
    length_hint: u32,
}

impl Default for MetadataDecoder {
    fn default() -> Self {
        Self { aggregated_length: 0, length_hint: 0 }
    }
}

impl MetadataDecoder {
    pub fn decode(&mut self, section: &mut PooledSection, data: Span<'_>) -> Status {
        let mut reader = ByteReader::new(data);
        let count = reader.read_u32_le().unwrap_or(0);
        self.aggregated_length = count;
        section.object_mut().payload.set_array_length(count);
        Status::Ok
    }

    pub fn set_length_hint(&mut self, hint: u32) { self.length_hint = hint; }
    pub fn aggregated_length(&self) -> u32 { self.aggregated_length.max(self.length_hint) }

    pub fn decode_fields(&self, data: Span<'_>, out: &mut Vec<MetadataField>) -> Status {
        out.clear();
        let mut reader = ByteReader::new(data);
        let count = match reader.read_u32_le() {
            Ok(v) => v,
            Err(e) => return e,
        };
        for _ in 0..count {
            let field_id = match reader.read_u16_le() { Ok(v) => v, Err(e) => return e };
            let type_code = match reader.read_u16_le() { Ok(v) => v, Err(e) => return e };
            let value_offset = match reader.read_u32_le() { Ok(v) => v, Err(e) => return e };
            let value_length = match reader.read_u32_le() { Ok(v) => v, Err(e) => return e };
            out.push(MetadataField { field_id, type_code, value_offset, value_length });
        }
        Status::Ok
    }
}
