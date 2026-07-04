//! Xref binding flag helpers and length merge utilities.

use crate::common::types::{SectionDescriptor, Status};
use crate::format::spec;

pub fn apply_binding_flags_00(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(0 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_00(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_01(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(1 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_01(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_02(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(2 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_02(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_03(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(3 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_03(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_04(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(4 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_04(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_05(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(5 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_05(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_06(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(6 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_06(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_07(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(7 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_07(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_08(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(8 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_08(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_09(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(9 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_09(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_10(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(10 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_10(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_11(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(11 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_11(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_12(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(12 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_12(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_13(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(13 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_13(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_14(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(14 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_14(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_15(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(15 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_15(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_16(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(16 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_16(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_17(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(17 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_17(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_18(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(18 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_18(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_19(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(19 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_19(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_20(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(20 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_20(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_21(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(21 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_21(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_22(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(22 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_22(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_23(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(23 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_23(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_24(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(24 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_24(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_25(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(25 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_25(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_26(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(26 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_26(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_27(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(27 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_27(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_28(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(28 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_28(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_29(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(29 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_29(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_30(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(30 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_30(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_31(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(31 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_31(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_32(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(32 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_32(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_33(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(33 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_33(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_34(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(34 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_34(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_35(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(35 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_35(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_36(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(36 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_36(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_37(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(37 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_37(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_38(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(38 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_38(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_39(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(39 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_39(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_40(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(40 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_40(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_41(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(41 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_41(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_42(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(42 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_42(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_43(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(43 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_43(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_44(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(44 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_44(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_45(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(45 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_45(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_46(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(46 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_46(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_47(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(47 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_47(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_48(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(48 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_48(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_49(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(49 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_49(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_50(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(50 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_50(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_51(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(51 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_51(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_52(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(52 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_52(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_53(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(53 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_53(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_54(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(54 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_54(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_55(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(55 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_55(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_56(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(56 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_56(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_57(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(57 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_57(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_58(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(58 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_58(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_59(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(59 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_59(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_60(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(60 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_60(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_61(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(61 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_61(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_62(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(62 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_62(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_63(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(63 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_63(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_64(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(64 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_64(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_65(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(65 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_65(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_66(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(66 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_66(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_67(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(67 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_67(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_68(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(68 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_68(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_69(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(69 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_69(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_70(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(70 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_70(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_71(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(71 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_71(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_72(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(72 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_72(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_73(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(73 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_73(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_74(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(74 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_74(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_75(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(75 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_75(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_76(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(76 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_76(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_77(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(77 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_77(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_78(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(78 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_78(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_79(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(79 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_79(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_80(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(80 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_80(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_81(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(81 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_81(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_82(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(82 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_82(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_83(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(83 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_83(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_84(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(84 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_84(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_85(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(85 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_85(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_86(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(86 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_86(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_87(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(87 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_87(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_88(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(88 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_88(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_89(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(89 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_89(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_90(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(90 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_90(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_91(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(91 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_91(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_92(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(92 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_92(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_93(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(93 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_93(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_94(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(94 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_94(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_95(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(95 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_95(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_96(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(96 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_96(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_97(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(97 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_97(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_98(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(98 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_98(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_99(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(99 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_99(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_100(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(100 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_100(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_101(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(101 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_101(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_102(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(102 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_102(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_103(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(103 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_103(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_104(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(104 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_104(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_105(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(105 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_105(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_106(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(106 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_106(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_107(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(107 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_107(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_108(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(108 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_108(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_109(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(109 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_109(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_110(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(110 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_110(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_111(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(111 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_111(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_112(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(112 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_112(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_113(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(113 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_113(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_114(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(114 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_114(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_115(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(115 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_115(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_116(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(116 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_116(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_117(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(117 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_117(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_118(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(118 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_118(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}

pub fn apply_binding_flags_119(desc: &mut SectionDescriptor, flags: u16) -> Status {
    if (flags & spec::BINDING_FLAG_LENGTH_OVERRIDE) != 0 {
        desc.extended_length = desc.length.saturating_add(119 as u32);
    }
    if (flags & spec::BINDING_FLAG_DEFERRED) != 0 {
        desc.flags |= crate::common::types::SectionFlags::DeferredChecksum as u8;
    }
    Status::Ok
}

pub fn merge_section_lengths_119(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(spec::MAX_EXTENDED_LENGTH)
}
