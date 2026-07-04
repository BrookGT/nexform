//! Secondary index table parser.


use crate::common::buffer::ByteReader;
use crate::common::types::{Span, Status};

#[derive(Debug, Clone, Copy, Default)]
pub struct IndexEntry {
    pub section_id: u16,
    pub file_offset: u32,
    pub byte_length: u32,
}

pub struct IndexTable {
    entries: Vec<IndexEntry>,
}

impl Default for IndexTable {
    fn default() -> Self { Self { entries: Vec::new() } }
}

impl IndexTable {
    pub fn parse(data: Span<'_>) -> Result<Self, Status> {
        let mut reader = ByteReader::new(data);
        let count = reader.read_u32_le().map_err(|_| Status::Incomplete)?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            entries.push(IndexEntry {
                section_id: reader.read_u16_le().map_err(|_| Status::Incomplete)?,
                file_offset: reader.read_u32_le().map_err(|_| Status::Incomplete)?,
                byte_length: reader.read_u32_le().map_err(|_| Status::Incomplete)?,
            });
        }
        Ok(Self { entries })
    }

    pub fn entries(&self) -> &[IndexEntry] { &self.entries }
}
