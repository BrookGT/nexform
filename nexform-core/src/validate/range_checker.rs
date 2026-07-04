//! Document byte range checker.

use crate::common::types::{has_flag, MetadataField, SectionDescriptor, SectionFlags, Status};

#[derive(Debug, Clone, Copy)]
pub enum RangeKind { SectionSpan, FieldSpan, IndexSpan }

#[derive(Debug, Clone, Copy)]
pub struct RangeSpec { pub kind: RangeKind, pub start: u32, pub end: u32, pub length: u32, pub owner_id: u16, pub field_id: u16, pub alignment: u8, pub allow_zero_length: bool }

#[derive(Debug, Clone, Copy)]
pub struct FieldSpan { pub field_id: u16, pub value_offset: u32, pub value_length: u32, pub alignment: u8 }

#[derive(Debug, Clone, Default)]
pub struct RangeCheckReport { pub document_size: u32, pub violations: u32 }

pub struct RangeChecker { ranges: Vec<RangeSpec>, report: RangeCheckReport, document_size: usize, strict: bool }

impl Default for RangeChecker { fn default() -> Self { Self { ranges: Vec::new(), report: RangeCheckReport::default(), document_size: 0, strict: false } } }

impl RangeChecker {
    pub fn clear(&mut self) { self.ranges.clear(); self.report = RangeCheckReport::default(); }
    pub fn set_document_size(&mut self, size: usize) { self.document_size = size; self.report.document_size = size as u32; }
    pub fn set_strict(&mut self, strict: bool) { self.strict = strict; }
    pub fn last_report(&self) -> &RangeCheckReport { &self.report }
    pub fn add_from_descriptors(&mut self, descriptors: &[SectionDescriptor]) { for d in descriptors { self.add_from_descriptor(d); } }
    pub fn add_from_descriptor(&mut self, desc: &SectionDescriptor) {
        self.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start: desc.offset, end: 0, length: desc.length, owner_id: desc.section_id, field_id: 0, alignment: 1, allow_zero_length: has_flag(desc.flags, SectionFlags::Optional) });
    }
    pub fn add_from_metadata_fields(&mut self, section_id: u16, fields: &[MetadataField], payload_base: u32) {
        for f in fields { self.ranges.push(RangeSpec { kind: RangeKind::FieldSpan, start: payload_base + f.value_offset, end: 0, length: f.value_length, owner_id: section_id, field_id: f.field_id, alignment: 1, allow_zero_length: false }); }
    }
    pub fn validate_all(&mut self) -> Status {
        for spec in self.ranges.clone() {
            let end = spec.start.saturating_add(spec.length);
            if end as usize > self.document_size && !spec.allow_zero_length { self.report.violations += 1; if self.strict { return Status::SectionOutOfRange; } }
        }
        Status::Ok
    }
}

pub fn validate_range_slice_00(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 0, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_01(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 1, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_02(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 2, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_03(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 3, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_04(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 4, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_05(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 5, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_06(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 6, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_07(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 7, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_08(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 8, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_09(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 9, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_10(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 10, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_11(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 11, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_12(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 12, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_13(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 13, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_14(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 14, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_15(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 15, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_16(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 16, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_17(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 17, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_18(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 18, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_19(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 19, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_20(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 20, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_21(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 21, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_22(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 22, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_23(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 23, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_24(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 24, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_25(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 25, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_26(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 26, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_27(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 27, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_28(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 28, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_29(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 29, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_30(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 30, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_31(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 31, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_32(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 32, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_33(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 33, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_34(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 34, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_35(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 35, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_36(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 36, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_37(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 37, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_38(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 38, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_39(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 39, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_40(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 40, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_41(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 41, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_42(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 42, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_43(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 43, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_44(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 44, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_45(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 45, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_46(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 46, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_47(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 47, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_48(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 48, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_49(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 49, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_50(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 50, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_51(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 51, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_52(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 52, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_53(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 53, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}

pub fn validate_range_slice_54(checker: &mut RangeChecker, start: u32, len: u32, owner: u16) -> Status {
    checker.ranges.push(RangeSpec { kind: RangeKind::SectionSpan, start, end: 0, length: len, owner_id: owner, field_id: 54, alignment: 1, allow_zero_length: false });
    checker.validate_all()
}
