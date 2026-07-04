//! NXF document header parser.


use crate::common::buffer::ByteReader;
use crate::common::types::{HeaderInfo, SectionDescriptor, Span, Status, SectionType};
use crate::format::spec;

macro_rules! rd {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return e,
        }
    };
}

pub struct HeaderParser;

impl HeaderParser {
    pub fn parse(input: Span<'_>, header: &mut HeaderInfo, sections: &mut Vec<SectionDescriptor>) -> Status {
        if input.len() < spec::HEADER_SIZE as usize {
            return Status::Incomplete;
        }
        let mut reader = ByteReader::new(input);
        header.magic = rd!(reader.read_u32_le());
        if header.magic != spec::MAGIC {
            return Status::InvalidMagic;
        }
        header.version_major = rd!(reader.read_u16_le());
        header.version_minor = rd!(reader.read_u16_le());
        if header.version_major != spec::VERSION_MAJOR {
            return Status::InvalidVersion;
        }
        header.document_flags = rd!(reader.read_u32_le());
        header.section_count = rd!(reader.read_u16_le());
        header.index_offset = rd!(reader.read_u32_le());
        header.creation_timestamp = rd!(reader.read_u32_le());
        let _ = reader.skip(10);
        sections.clear();
        let table_offset = if header.index_offset > 0 {
            header.index_offset as usize
        } else {
            spec::HEADER_SIZE as usize
        };
        if table_offset >= input.len() {
            return Status::SectionOutOfRange;
        }
        let mut tr = ByteReader::new(Span::new(&input.data[table_offset..]));
        for _ in 0..header.section_count {
            let mut desc = SectionDescriptor::default();
            let ty = rd!(tr.read_u8());
            desc.ty = SectionType::try_from(ty).unwrap_or(SectionType::Reserved);
            desc.flags = rd!(tr.read_u8());
            desc.section_id = rd!(tr.read_u16_le());
            desc.offset = rd!(tr.read_u32_le());
            desc.length = rd!(tr.read_u32_le());
            desc.checksum = rd!(tr.read_u32_le());
            desc.extended_length = rd!(tr.read_u32_le());
            desc.xref_target = rd!(tr.read_u16_le());
            sections.push(desc);
        }
        Status::Ok
    }
}

impl TryFrom<u8> for SectionType {
    type Error = ();
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0x01 => SectionType::HeaderInfo,
            0x02 => SectionType::Metadata,
            0x03 => SectionType::Payload,
            0x04 => SectionType::Xref,
            0x05 => SectionType::Index,
            0x06 => SectionType::Annotation,
            0x07 => SectionType::CompressedBlob,
            0x08 => SectionType::StateInit,
            0x09 => SectionType::TransformChain,
            0x0A => SectionType::DeferredValidate,
            0x0B => SectionType::ObjectGraph,
            _ => SectionType::Reserved,
        })
    }
}
