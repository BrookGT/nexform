//! NXF document writer for round-trip export.


use crate::common::buffer::{ByteWriter, GrowableBuffer};
use crate::common::checksum::crc32;
use crate::common::types::{HeaderInfo, SectionDescriptor, SectionType, Status};
use crate::format::spec;

pub struct DocumentWriter {
    buffer: GrowableBuffer,
}

impl Default for DocumentWriter {
    fn default() -> Self { Self { buffer: GrowableBuffer::default() } }
}

impl DocumentWriter {
    pub fn begin(&mut self, header: &HeaderInfo, sections: &[SectionDescriptor]) -> Status {
        self.buffer.clear();
        let mut writer = ByteWriter::bind(&mut self.buffer);
        if writer.write_u32_le(spec::MAGIC) != Status::Ok { return Status::InternalError; }
        if writer.write_u16_le(header.version_major) != Status::Ok { return Status::InternalError; }
        if writer.write_u16_le(header.version_minor) != Status::Ok { return Status::InternalError; }
        if writer.write_u32_le(header.document_flags) != Status::Ok { return Status::InternalError; }
        if writer.write_u16_le(sections.len() as u16) != Status::Ok { return Status::InternalError; }
        if writer.write_u32_le(spec::HEADER_SIZE) != Status::Ok { return Status::InternalError; }
        if writer.write_u32_le(header.creation_timestamp) != Status::Ok { return Status::InternalError; }
        if writer.write_zeroes(10) != Status::Ok { return Status::InternalError; }
        for desc in sections {
            if writer.write_u8(desc.ty as u8) != Status::Ok { return Status::InternalError; }
            if writer.write_u8(desc.flags) != Status::Ok { return Status::InternalError; }
            if writer.write_u16_le(desc.section_id) != Status::Ok { return Status::InternalError; }
            if writer.write_u32_le(desc.offset) != Status::Ok { return Status::InternalError; }
            if writer.write_u32_le(desc.length) != Status::Ok { return Status::InternalError; }
            if writer.write_u32_le(desc.checksum) != Status::Ok { return Status::InternalError; }
            if writer.write_u32_le(desc.extended_length) != Status::Ok { return Status::InternalError; }
            if writer.write_u16_le(desc.xref_target) != Status::Ok { return Status::InternalError; }
        }
        Status::Ok
    }

    pub fn finish(&mut self) -> &[u8] { self.buffer.data() }
}
