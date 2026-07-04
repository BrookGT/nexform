//! NXF format constants and flag definitions.


pub const MAGIC: u32 = 0x3158_464E;
pub const VERSION_MAJOR: u16 = 2;
pub const VERSION_MINOR: u16 = 4;
pub const MAX_SECTIONS: u32 = 256;
pub const MAX_SECTION_SIZE: u32 = 16 * 1024 * 1024;
pub const MAX_EXTENDED_LENGTH: u32 = 64 * 1024 * 1024;
pub const HEADER_SIZE: u32 = 32;
pub const SECTION_DESCRIPTOR_SIZE: u32 = 20;
pub const XREF_ENTRY_SIZE: u32 = 12;
pub const METADATA_FIELD_SIZE: u32 = 12;
pub const OBJECT_HEADER_SIZE: u32 = 8;

pub const DOCUMENT_FLAG_STREAMING: u8 = 0x01;
pub const DOCUMENT_FLAG_INDEXED: u8 = 0x02;
pub const DOCUMENT_FLAG_RECOVERABLE: u8 = 0x04;
pub const DOCUMENT_FLAG_POLYMORPHIC: u8 = 0x08;

pub const BINDING_FLAG_LENGTH_OVERRIDE: u16 = 0x01;
pub const BINDING_FLAG_TYPE_REDIRECT: u16 = 0x02;
pub const BINDING_FLAG_DEFERRED: u16 = 0x04;

pub const RECOVERY_MAGIC: u32 = 0x5243_5652;
