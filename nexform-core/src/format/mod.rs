//! NXF on-disk format: header, sections, metadata, xref, writer.

pub mod bindings;
pub mod annotation_parser;
pub mod document_writer;
pub mod header;
pub mod index_table;
pub mod integrity_checker;
pub mod metadata;
pub mod section;
pub mod section_catalog;
pub mod spec;
pub mod timeline;
pub mod timeline_export;
pub mod xref;
