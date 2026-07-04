//! Document profile tables for NXF validation.

use crate::common::types::{HeaderInfo, Status};
use crate::format::spec;

pub fn profile_check_streaming_00(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (0 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_00(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_01(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (1 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_01(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_02(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (2 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_02(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_03(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (3 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_03(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_04(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (4 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_04(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_05(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (5 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_05(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_06(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (6 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_06(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_07(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (7 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_07(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_08(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (8 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_08(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_09(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (9 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_09(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_10(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (10 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_10(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_11(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (11 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_11(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_12(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (12 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_12(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_13(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (13 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_13(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_14(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (14 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_14(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_15(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (15 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_15(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_16(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (16 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_16(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_17(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (17 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_17(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_18(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (18 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_18(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_19(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (19 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_19(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_20(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (20 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_20(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_21(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (21 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_21(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_22(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (22 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_22(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_23(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (23 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_23(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_24(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (24 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_24(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_25(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (25 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_25(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_26(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (26 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_26(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_27(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (27 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_27(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_28(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (28 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_28(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_29(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (29 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_29(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_30(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (30 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_30(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_31(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (31 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_31(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_32(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (32 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_32(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_33(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (33 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_33(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_34(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (34 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_34(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_35(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (35 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_35(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_36(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (36 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_36(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_37(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (37 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_37(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_38(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (38 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_38(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_39(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (39 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_39(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_40(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (40 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_40(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_41(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (41 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_41(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_42(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (42 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_42(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_43(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (43 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_43(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_44(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (44 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_44(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_45(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (45 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_45(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_46(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (46 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_46(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_47(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (47 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_47(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_48(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (48 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_48(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_49(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (49 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_49(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_50(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (50 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_50(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_51(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (51 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_51(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_52(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (52 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_52(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_53(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (53 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_53(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_54(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (54 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_54(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_55(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (55 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_55(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_56(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (56 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_56(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_57(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (57 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_57(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_58(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (58 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_58(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_59(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (59 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_59(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_60(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (60 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_60(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_61(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (61 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_61(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_62(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (62 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_62(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_63(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (63 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_63(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_64(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (64 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_64(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_65(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (65 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_65(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_66(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (66 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_66(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_67(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (67 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_67(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_68(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (68 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_68(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_69(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (69 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_69(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_70(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (70 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_70(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_71(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (71 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_71(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_72(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (72 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_72(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_73(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (73 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_73(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_74(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (74 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_74(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_75(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (75 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_75(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_76(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (76 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_76(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_77(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (77 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_77(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_78(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (78 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_78(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_79(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (79 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_79(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_80(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (80 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_80(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_81(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (81 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_81(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_82(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (82 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_82(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_83(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (83 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_83(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_84(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (84 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_84(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_85(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (85 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_85(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_86(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (86 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_86(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_87(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (87 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_87(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_88(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (88 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_88(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_89(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (89 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_89(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_90(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (90 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_90(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_91(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (91 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_91(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_92(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (92 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_92(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_93(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (93 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_93(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_94(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (94 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_94(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_95(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (95 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_95(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_96(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (96 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_96(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_97(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (97 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_97(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_98(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (98 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_98(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn profile_check_streaming_99(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_STREAMING as u32) == 0 { return Status::SchemaViolation; }
    if header.section_count > (99 as u16 + 1) { Status::Ok } else { Status::Incomplete }
}

pub fn profile_check_recoverable_99(header: &HeaderInfo) -> Status {
    if (header.document_flags & spec::DOCUMENT_FLAG_RECOVERABLE as u32) == 0 { return Status::SchemaViolation; }
    Status::Ok
}
