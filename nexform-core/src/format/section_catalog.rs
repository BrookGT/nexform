//! Section layout catalog validation.


use crate::common::types::{SectionDescriptor, SectionType, Status};
use crate::format::spec;

pub struct SectionCatalog;

impl SectionCatalog {
    pub fn validate_document_layout(&self, sections: &[SectionDescriptor]) -> Status {
        if sections.len() > spec::MAX_SECTIONS as usize {
            return Status::SchemaViolation;
        }
        let mut seen = alloc::collections::BTreeSet::new();
        for desc in sections {
            if !seen.insert(desc.section_id) {
                return Status::SchemaViolation;
            }
            if desc.length > spec::MAX_SECTION_SIZE {
                return Status::SectionOutOfRange;
            }
            if desc.ty == SectionType::Reserved {
                return Status::SchemaViolation;
            }
        }
        Status::Ok
    }
}
