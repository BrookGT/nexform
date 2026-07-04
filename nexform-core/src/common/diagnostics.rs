//! Structured diagnostics and parse error reporting.

use crate::common::types::{Status, SectionType};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: u16,
    pub status: Status,
    pub section_id: u16,
    pub message: String,
}

pub struct DiagnosticContext {
    entries: Vec<Diagnostic>,
    max_entries: usize,
}

impl Default for DiagnosticContext {
    fn default() -> Self {
        Self { entries: Vec::new(), max_entries: 256 }
    }
}

impl DiagnosticContext {
    pub fn new(max: usize) -> Self {
        Self { entries: Vec::new(), max_entries: max }
    }

    pub fn push(&mut self, status: Status, section_id: u16, message: impl Into<String>) {
        if self.entries.len() >= self.max_entries { return; }
        self.entries.push(Diagnostic { code: status as u16, status, section_id, message: message.into() });
    }

    pub fn entries(&self) -> &[Diagnostic] { &self.entries }

    pub fn clear(&mut self) { self.entries.clear(); }
}

pub fn format_section_diag_00(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [00]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_00(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (0 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_01(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [01]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_01(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (1 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_02(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [02]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_02(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (2 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_03(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [03]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_03(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (3 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_04(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [04]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_04(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (4 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_05(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [05]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_05(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (5 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_06(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [06]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_06(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (6 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_07(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [07]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_07(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (7 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_08(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [08]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_08(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (8 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_09(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [09]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_09(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (9 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_10(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [10]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_10(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (10 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_11(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [11]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_11(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (11 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_12(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [12]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_12(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (12 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_13(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [13]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_13(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (13 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_14(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [14]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_14(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (14 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_15(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [15]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_15(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (15 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_16(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [16]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_16(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (16 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_17(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [17]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_17(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (17 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_18(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [18]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_18(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (18 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_19(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [19]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_19(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (19 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_20(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [20]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_20(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (20 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_21(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [21]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_21(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (21 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_22(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [22]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_22(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (22 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_23(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [23]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_23(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (23 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_24(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [24]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_24(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (24 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_25(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [25]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_25(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (25 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_26(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [26]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_26(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (26 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_27(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [27]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_27(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (27 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_28(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [28]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_28(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (28 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_29(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [29]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_29(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (29 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_30(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [30]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_30(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (30 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_31(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [31]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_31(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (31 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_32(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [32]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_32(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (32 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_33(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [33]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_33(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (33 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_34(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [34]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_34(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (34 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_35(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [35]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_35(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (35 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_36(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [36]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_36(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (36 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_37(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [37]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_37(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (37 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_38(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [38]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_38(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (38 % 7) as u8 + 4,
    }
}

pub fn format_section_diag_39(ty: SectionType, section_id: u16, detail: &str) -> String {
    alloc::format!("section {} id={} [39]: {}", crate::common::types::section_type_to_string(ty), section_id, detail)
}

pub fn classify_status_39(status: Status) -> u8 {
    match status {
        Status::Ok => 0,
        Status::Incomplete => 1,
        Status::ChecksumMismatch => 2,
        Status::SchemaViolation => 3,
        _ => (39 % 7) as u8 + 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diag() {
        let mut ctx = DiagnosticContext::default();
        ctx.push(Status::Ok, 1, "test");
        assert_eq!(ctx.entries().len(), 1);
    }
}
