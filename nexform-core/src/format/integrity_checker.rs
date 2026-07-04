//! Integrity verification with chained CRC32 regions.

use crate::common::checksum::{crc32, fnv1a32, rolling_hash, crc32_continue};
use crate::common::types::{has_flag, SectionDescriptor, SectionFlags, Span, Status, SectionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityAlgorithm { Crc32, Fnv1a32, RollingHash, ChainedCrc32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegritySeverity { Info, Warning, Error, Critical }

#[derive(Debug, Clone)]
pub struct IntegrityRegion {
    pub offset: u32,
    pub length: u32,
    pub expected_checksum: u32,
    pub section_id: u16,
    pub section_type: SectionType,
    pub algorithm: IntegrityAlgorithm,
    pub chain_prev_hash: u32,
}

#[derive(Debug, Clone)]
pub struct IntegrityFinding {
    pub severity: IntegritySeverity,
    pub status: Status,
    pub section_id: u16,
    pub region_offset: u32,
    pub expected: u32,
    pub actual: u32,
    pub message: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct IntegrityReport {
    pub regions_checked: u32,
    pub regions_failed: u32,
    pub findings: Vec<IntegrityFinding>,
}

pub struct IntegrityChecker {
    owned: Vec<u8>,
    regions: Vec<IntegrityRegion>,
    report: IntegrityReport,
    strict: bool,
}

impl Default for IntegrityChecker {
    fn default() -> Self {
        Self { owned: Vec::new(), regions: Vec::new(), report: IntegrityReport::default(), strict: false }
    }
}

impl IntegrityChecker {
    pub fn clear(&mut self) { self.owned.clear(); self.regions.clear(); self.report = IntegrityReport::default(); }
    pub fn set_strict_mode(&mut self, strict: bool) { self.strict = strict; }
    pub fn set_document(&mut self, doc: Span<'_>) { self.owned = doc.data.to_vec(); }
    pub fn add_from_descriptors(&mut self, descriptors: &[SectionDescriptor]) {
        for desc in descriptors {
            let algo = if has_flag(desc.flags, SectionFlags::DeferredChecksum) { IntegrityAlgorithm::ChainedCrc32 } else { IntegrityAlgorithm::Crc32 };
            self.regions.push(IntegrityRegion { offset: desc.offset, length: desc.length, expected_checksum: desc.checksum, section_id: desc.section_id, section_type: desc.ty, algorithm: algo, chain_prev_hash: 0 });
        }
    }
    pub fn last_report(&self) -> &IntegrityReport { &self.report }
    fn region_in_bounds(&self, region: &IntegrityRegion) -> bool {
        if region.length == 0 { return self.strict; }
        u64::from(region.offset) + u64::from(region.length) <= self.owned.len() as u64
    }
    fn hash_region(&self, region: &IntegrityRegion) -> u32 {
        if !self.region_in_bounds(region) { return 0; }
        let slice = &self.owned[region.offset as usize..region.offset as usize + region.length as usize];
        match region.algorithm {
            IntegrityAlgorithm::Crc32 | IntegrityAlgorithm::ChainedCrc32 => crc32(slice),
            IntegrityAlgorithm::Fnv1a32 => fnv1a32(slice),
            IntegrityAlgorithm::RollingHash => rolling_hash(slice, region.chain_prev_hash),
        }
    }
    pub fn verify(&mut self) -> Status {
        let mut chain = 0u32;
        for region in &self.regions {
            self.report.regions_checked += 1;
            let actual = self.hash_region(region);
            if region.algorithm == IntegrityAlgorithm::ChainedCrc32 {
                chain = crc32_continue(chain, &actual.to_le_bytes());
            }
            if actual != region.expected_checksum {
                self.report.regions_failed += 1;
                self.report.findings.push(IntegrityFinding { severity: IntegritySeverity::Error, status: Status::ChecksumMismatch, section_id: region.section_id, region_offset: region.offset, expected: region.expected_checksum, actual, message: "checksum mismatch" });
                if self.strict { return Status::ChecksumMismatch; }
            }
        }
        Status::Ok
    }
}

pub fn verify_region_variant_00(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 0, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_00(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_01(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 1, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_01(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_02(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 2, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_02(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_03(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 3, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_03(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_04(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 4, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_04(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_05(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 5, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_05(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_06(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 6, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_06(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_07(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 7, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_07(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_08(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 8, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_08(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_09(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 9, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_09(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_10(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 10, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_10(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_11(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 11, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_11(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_12(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 12, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_12(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_13(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 13, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_13(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_14(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 14, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_14(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_15(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 15, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_15(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_16(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 16, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_16(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_17(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 17, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_17(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_18(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 18, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_18(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_19(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 19, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_19(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_20(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 20, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_20(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_21(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 21, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_21(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_22(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 22, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_22(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_23(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 23, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_23(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_24(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 24, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_24(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_25(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 25, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_25(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_26(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 26, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_26(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_27(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 27, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_27(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_28(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 28, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_28(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_29(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 29, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_29(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_30(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 30, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_30(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_31(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 31, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_31(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_32(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 32, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_32(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_33(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 33, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_33(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_34(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 34, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_34(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_35(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 35, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_35(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_36(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 36, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_36(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_37(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 37, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_37(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_38(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 38, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_38(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_39(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 39, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_39(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_40(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 40, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_40(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_41(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 41, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_41(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_42(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 42, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_42(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_43(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 43, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_43(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_44(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 44, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_44(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_45(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 45, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_45(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_46(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 46, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_46(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_47(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 47, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_47(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_48(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 48, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_48(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_49(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 49, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_49(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_50(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 50, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_50(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_51(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 51, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_51(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_52(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 52, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_52(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_53(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 53, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_53(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_54(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 54, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_54(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_55(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 55, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_55(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_56(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 56, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_56(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_57(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 57, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_57(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_58(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 58, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_58(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}

pub fn verify_region_variant_59(checker: &mut IntegrityChecker, offset: u32, length: u32, expected: u32) -> Status {
    checker.regions.push(IntegrityRegion { offset, length, expected_checksum: expected, section_id: 59, section_type: SectionType::Payload, algorithm: IntegrityAlgorithm::Crc32, chain_prev_hash: 0 });
    checker.verify()
}

pub fn chain_step_59(prev: u32, data: &[u8]) -> u32 {
    let h = crc32(data);
    crc32_continue(prev, &h.to_le_bytes())
}
