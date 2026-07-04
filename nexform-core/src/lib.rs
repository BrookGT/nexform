//! nexform — Rust library for NXF (Nexus Exchange Format) document bundles.


#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;

pub mod api;
pub mod common;
pub mod core;
pub mod decode;
pub mod format;
pub mod runtime;
pub mod validate;

/// Public `nex::` namespace — Nexus Exchange format API surface.
pub mod nex {
    pub use crate::api::export_api::{export_document, ExportFormat, ExportOptions};
    pub use crate::api::parser::{decode, parse, process_state_input, Parser};
    pub use crate::api::session::Session;
    pub use crate::api::stream_api::StreamSession;
    pub use crate::common::types::*;
}

pub use nex::*;
