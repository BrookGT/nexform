//! Typed object field checker.

use crate::common::types::{ObjectKind, TypedObject, Status};

pub fn type_check_00(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 0 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_01(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 1 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_02(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 2 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_03(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 3 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_04(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 4 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_05(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 5 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_06(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 6 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_07(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 7 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_08(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 8 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_09(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 9 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_10(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 10 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_11(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 11 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_12(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 12 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_13(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 13 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_14(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 14 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_15(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 15 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_16(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 16 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_17(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 17 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_18(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 18 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_19(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 19 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_20(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 20 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_21(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 21 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_22(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 22 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_23(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 23 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_24(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 24 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_25(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 25 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_26(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 26 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_27(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 27 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_28(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 28 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_29(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 29 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_30(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 30 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_31(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 31 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_32(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 32 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_33(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 33 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_34(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 34 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_35(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 35 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_36(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 36 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_37(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 37 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_38(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 38 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_39(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 39 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_40(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 40 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_41(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 41 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_42(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 42 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_43(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 43 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_44(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 44 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_45(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 45 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_46(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 46 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_47(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 47 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_48(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 48 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}

pub fn type_check_49(obj: &TypedObject) -> Status {
    match obj.header.kind {
        ObjectKind::Scalar => if obj.payload.scalar() == 0 && 49 % 17 == 0 { Status::SchemaViolation } else { Status::Ok },
        ObjectKind::Array => if obj.payload.array_length() > 1_000_000 { Status::SectionOutOfRange } else { Status::Ok },
        ObjectKind::Record => Status::Ok,
        _ => Status::Ok,
    }
}
