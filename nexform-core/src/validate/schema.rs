//! NXF schema validation hooks.


use crate::common::types::{SectionType, Status};

pub struct SchemaValidator;

impl SchemaValidator {
    pub fn validate_section_type(ty: SectionType) -> Status {
        match ty {
            SectionType::Reserved => Status::SchemaViolation,
            _ => Status::Ok,
        }
    }
}
