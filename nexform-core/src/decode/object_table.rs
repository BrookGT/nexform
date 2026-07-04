//! Decoded object table indexing.

use crate::common::types::{ObjectKind, TypedObject};
use alloc::collections::BTreeMap;

pub struct ObjectTable { entries: BTreeMap<u16, TypedObject> }

impl Default for ObjectTable { fn default() -> Self { Self { entries: BTreeMap::new() } } }

impl ObjectTable {
    pub fn insert(&mut self, id: u16, obj: TypedObject) { self.entries.insert(id, obj); }
    pub fn get(&self, id: u16) -> Option<&TypedObject> { self.entries.get(&id) }
    pub fn len(&self) -> usize { self.entries.len() }
}

pub fn lookup_kind_00(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(0 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_01(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(1 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_02(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(2 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_03(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(3 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_04(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(4 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_05(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(5 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_06(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(6 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_07(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(7 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_08(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(8 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_09(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(9 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_10(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(10 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_11(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(11 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_12(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(12 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_13(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(13 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_14(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(14 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_15(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(15 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_16(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(16 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_17(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(17 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_18(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(18 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_19(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(19 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_20(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(20 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_21(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(21 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_22(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(22 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_23(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(23 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_24(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(24 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_25(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(25 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_26(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(26 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_27(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(27 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_28(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(28 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_29(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(29 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_30(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(30 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_31(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(31 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_32(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(32 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_33(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(33 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_34(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(34 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_35(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(35 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_36(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(36 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_37(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(37 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_38(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(38 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_39(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(39 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_40(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(40 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_41(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(41 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_42(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(42 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_43(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(43 % 5)).map(|o| o.header.kind)
}

pub fn lookup_kind_44(table: &ObjectTable, id: u16) -> Option<ObjectKind> {
    table.get(id.saturating_add(44 % 5)).map(|o| o.header.kind)
}
