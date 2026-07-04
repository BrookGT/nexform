//! Document profile validation.


use crate::common::types::{HeaderInfo, Status};
use crate::format::spec;

#[derive(Debug, Clone, Copy)]
pub enum DocumentProfile { Minimal, Standard, Streaming, Recoverable }

pub struct ProfileValidator { profile: DocumentProfile }

impl Default for ProfileValidator {
    fn default() -> Self { Self { profile: DocumentProfile::Standard } }
}

impl ProfileValidator {
    pub fn detect_profile(header: &HeaderInfo) -> DocumentProfile {
        if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) != 0 {
            DocumentProfile::Recoverable
        } else if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) != 0 {
            DocumentProfile::Streaming
        } else if header.section_count <= 2 {
            DocumentProfile::Minimal
        } else {
            DocumentProfile::Standard
        }
    }

    pub fn set_profile(&mut self, profile: DocumentProfile) { self.profile = profile; }

    pub fn validate_header(&self, header: &HeaderInfo) -> Status {
        if header.magic != spec::MAGIC { return Status::InvalidMagic; }
        if header.version_major != spec::VERSION_MAJOR { return Status::InvalidVersion; }
        match self.profile {
            DocumentProfile::Streaming if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 => Status::SchemaViolation,
            DocumentProfile::Recoverable if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 => Status::SchemaViolation,
            _ => Status::Ok,
        }
    }
}
