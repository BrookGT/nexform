//! Core NXF value types, section descriptors, and status codes.


use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Ok = 0,
    Incomplete,
    InvalidMagic,
    InvalidVersion,
    ChecksumMismatch,
    SectionOutOfRange,
    XrefUnresolved,
    SchemaViolation,
    StateError,
    AllocationFailed,
    RecoveryFailed,
    DecoderError,
    InternalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionType {
    HeaderInfo = 0x01,
    Metadata = 0x02,
    Payload = 0x03,
    Xref = 0x04,
    Index = 0x05,
    Annotation = 0x06,
    CompressedBlob = 0x07,
    StateInit = 0x08,
    TransformChain = 0x09,
    DeferredValidate = 0x0A,
    ObjectGraph = 0x0B,
    Reserved = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionFlags {
    None = 0x00,
    Optional = 0x01,
    Compressed = 0x02,
    Encrypted = 0x04,
    Recoverable = 0x08,
    MergeLengths = 0x10,
    DeferredChecksum = 0x20,
    CrossLinked = 0x40,
    Polymorphic = 0x80,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InterpretMode {
    Raw = 0,
    Structured,
    Graph,
    Stream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ObjectKind {
    None = 0,
    Scalar,
    Array,
    Record,
    StreamHandle,
    GraphNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecoveryAction {
    None = 0,
    RewindSection,
    ReleasePoolSlot,
    ResetArena,
    InvalidateCache,
    ReplayDecode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span<'a> {
    pub data: &'a [u8],
}

impl<'a> Span<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[derive(Debug)]
pub struct MutableSpan<'a> {
    pub data: &'a mut [u8],
    pub size: usize,
    pub capacity: usize,
}

impl<'a> MutableSpan<'a> {
    pub fn empty() -> Self {
        Self { data: &mut [], size: 0, capacity: 0 }
    }
}

impl Default for SectionType {
    fn default() -> Self { SectionType::Reserved }
}

impl Default for InterpretMode {
    fn default() -> Self { InterpretMode::Raw }
}

impl Default for ObjectKind {
    fn default() -> Self { ObjectKind::None }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SectionDescriptor {
    pub ty: SectionType,
    pub flags: u8,
    pub section_id: u16,
    pub offset: u32,
    pub length: u32,
    pub checksum: u32,
    pub extended_length: u32,
    pub xref_target: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct XrefEntry {
    pub source_id: u16,
    pub target_id: u16,
    pub field_offset: u32,
    pub resolved_length: u32,
    pub binding_flags: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MetadataField {
    pub field_id: u16,
    pub type_code: u16,
    pub value_offset: u32,
    pub value_length: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HeaderInfo {
    pub magic: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub document_flags: u32,
    pub section_count: u16,
    pub index_offset: u32,
    pub creation_timestamp: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DecodeContext {
    pub mode: InterpretMode,
    pub active_section: u16,
    pub stream_generation: u32,
    pub recovery_pending: bool,
    pub defer_validation: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationTicket {
    pub section_id: u16,
    pub expected_checksum: u32,
    pub byte_range_start: u32,
    pub byte_range_end: u32,
    pub consumed: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectHeader {
    pub kind: ObjectKind,
    pub payload_words: u32,
    pub type_tag: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectPayload {
    pub raw: [u8; 16],
}

impl ObjectPayload {
    pub fn set_scalar(&mut self, v: u64) {
        self.raw[0..8].copy_from_slice(&v.to_le_bytes());
    }

    pub fn scalar(&self) -> u64 {
        u64::from_le_bytes(self.raw[0..8].try_into().unwrap_or([0; 8]))
    }

    pub fn set_array_length(&mut self, v: u32) {
        self.raw[0..4].copy_from_slice(&v.to_le_bytes());
    }

    pub fn array_length(&self) -> u32 {
        u32::from_le_bytes(self.raw[0..4].try_into().unwrap_or([0; 4]))
    }

    pub fn set_record_fields(&mut self, v: u32) {
        self.raw[0..4].copy_from_slice(&v.to_le_bytes());
    }

    pub fn record_fields(&self) -> u32 {
        u32::from_le_bytes(self.raw[0..4].try_into().unwrap_or([0; 4]))
    }

    pub fn set_stream_id(&mut self, v: u32) {
        self.raw[0..4].copy_from_slice(&v.to_le_bytes());
    }

    pub fn stream_id(&self) -> u32 {
        u32::from_le_bytes(self.raw[0..4].try_into().unwrap_or([0; 4]))
    }
}

#[derive(Debug, Default)]
pub struct TypedObject {
    pub header: ObjectHeader,
    pub payload: ObjectPayload,
    pub heap_extension: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParseStats {
    pub bytes_consumed: usize,
    pub sections_parsed: usize,
    pub xrefs_resolved: usize,
    pub recovery_count: usize,
    pub decode_ops: usize,
}

pub fn has_flag(flags: u8, flag: SectionFlags) -> bool {
    (flags & flag as u8) != 0
}

pub fn status_to_string(status: Status) -> &'static str {
    match status {
        Status::Ok => "ok",
        Status::Incomplete => "incomplete",
        Status::InvalidMagic => "invalid_magic",
        Status::InvalidVersion => "invalid_version",
        Status::ChecksumMismatch => "checksum_mismatch",
        Status::SectionOutOfRange => "section_out_of_range",
        Status::XrefUnresolved => "xref_unresolved",
        Status::SchemaViolation => "schema_violation",
        Status::StateError => "state_error",
        Status::AllocationFailed => "allocation_failed",
        Status::RecoveryFailed => "recovery_failed",
        Status::DecoderError => "decoder_error",
        Status::InternalError => "internal_error",
    }
}

pub fn section_type_to_string(ty: SectionType) -> &'static str {
    match ty {
        SectionType::HeaderInfo => "header_info",
        SectionType::Metadata => "metadata",
        SectionType::Payload => "payload",
        SectionType::Xref => "xref",
        SectionType::Index => "index",
        SectionType::Annotation => "annotation",
        SectionType::CompressedBlob => "compressed_blob",
        SectionType::StateInit => "state_init",
        SectionType::TransformChain => "transform_chain",
        SectionType::DeferredValidate => "deferred_validate",
        SectionType::ObjectGraph => "object_graph",
        SectionType::Reserved => "reserved",
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", status_to_string(*self))
    }
}
