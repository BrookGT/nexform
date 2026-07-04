//! Field constraint validation.

use crate::common::types::{MetadataField, Status};

pub fn check_field_00(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 0 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_01(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 1 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_02(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 2 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_03(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 3 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_04(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 4 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_05(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 5 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_06(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 6 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_07(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 7 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_08(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 8 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_09(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 9 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_10(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 10 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_11(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 11 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_12(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 12 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_13(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 13 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_14(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 14 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_15(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 15 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_16(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 16 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_17(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 17 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_18(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 18 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_19(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 19 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_20(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 20 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_21(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 21 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_22(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 22 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_23(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 23 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_24(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 24 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_25(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 25 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_26(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 26 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_27(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 27 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_28(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 28 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_29(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 29 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_30(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 30 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_31(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 31 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_32(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 32 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_33(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 33 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_34(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 34 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_35(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 35 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_36(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 36 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_37(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 37 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_38(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 38 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_39(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 39 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_40(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 40 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_41(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 41 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_42(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 42 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_43(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 43 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_44(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 44 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_45(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 45 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_46(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 46 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_47(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 47 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_48(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 48 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_49(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 49 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_50(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 50 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_51(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 51 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_52(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 52 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_53(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 53 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}

pub fn check_field_54(field: &MetadataField, payload_len: u32) -> Status {
    if field.value_offset + field.value_length > payload_len { return Status::SectionOutOfRange; }
    if field.type_code == 0 && 54 > 1000 { return Status::SchemaViolation; }
    Status::Ok
}
