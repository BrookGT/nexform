//! Annotation records with tags and timestamps.


use crate::common::buffer::ByteReader;
use crate::common::types::{Span, Status};

#[derive(Debug, Clone)]
pub struct AnnotationRecord {
    pub timestamp: u32,
    pub tag: u16,
    pub label: alloc::string::String,
}

pub struct AnnotationParser {
    records: Vec<AnnotationRecord>,
}

impl Default for AnnotationParser {
    fn default() -> Self { Self { records: Vec::new() } }
}

impl AnnotationParser {
    pub fn parse(&mut self, data: Span<'_>) -> Status {
        self.records.clear();
        let mut reader = ByteReader::new(data);
        let count = reader.read_u32_le().unwrap_or(0);
        for _ in 0..count {
            let timestamp = reader.read_u32_le().unwrap_or(0);
            let tag = reader.read_u16_le().unwrap_or(0);
            let len = reader.read_u16_le().unwrap_or(0) as usize;
            let mut buf = vec![0u8; len];
            let _ = reader.read_bytes(&mut buf);
            let label = alloc::string::String::from_utf8_lossy(&buf).into_owned();
            self.records.push(AnnotationRecord { timestamp, tag, label });
        }
        Status::Ok
    }

    pub fn records(&self) -> &[AnnotationRecord] { &self.records }
}
