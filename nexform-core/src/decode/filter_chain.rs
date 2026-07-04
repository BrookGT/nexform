//! Decode filter chain stages.

use crate::common::types::{Span, Status};

pub type FilterFn = fn(Span<'_>, &mut [u8]) -> Status;

pub struct FilterChain { stages: Vec<FilterFn> }

impl Default for FilterChain { fn default() -> Self { Self { stages: Vec::new() } } }

impl FilterChain {
    pub fn push(&mut self, f: FilterFn) { self.stages.push(f); }
    pub fn run(&self, input: Span<'_>, out: &mut [u8]) -> Status {
        for stage in &self.stages { if stage(input, out) != Status::Ok { return Status::DecoderError; } }
        Status::Ok
    }
}

pub fn filter_stage_00(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (0 as u8); }
    Status::Ok
}

pub fn filter_stage_01(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (1 as u8); }
    Status::Ok
}

pub fn filter_stage_02(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (2 as u8); }
    Status::Ok
}

pub fn filter_stage_03(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (3 as u8); }
    Status::Ok
}

pub fn filter_stage_04(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (4 as u8); }
    Status::Ok
}

pub fn filter_stage_05(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (5 as u8); }
    Status::Ok
}

pub fn filter_stage_06(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (6 as u8); }
    Status::Ok
}

pub fn filter_stage_07(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (7 as u8); }
    Status::Ok
}

pub fn filter_stage_08(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (8 as u8); }
    Status::Ok
}

pub fn filter_stage_09(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (9 as u8); }
    Status::Ok
}

pub fn filter_stage_10(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (10 as u8); }
    Status::Ok
}

pub fn filter_stage_11(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (11 as u8); }
    Status::Ok
}

pub fn filter_stage_12(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (12 as u8); }
    Status::Ok
}

pub fn filter_stage_13(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (13 as u8); }
    Status::Ok
}

pub fn filter_stage_14(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (14 as u8); }
    Status::Ok
}

pub fn filter_stage_15(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (15 as u8); }
    Status::Ok
}

pub fn filter_stage_16(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (16 as u8); }
    Status::Ok
}

pub fn filter_stage_17(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (17 as u8); }
    Status::Ok
}

pub fn filter_stage_18(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (18 as u8); }
    Status::Ok
}

pub fn filter_stage_19(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (19 as u8); }
    Status::Ok
}

pub fn filter_stage_20(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (20 as u8); }
    Status::Ok
}

pub fn filter_stage_21(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (21 as u8); }
    Status::Ok
}

pub fn filter_stage_22(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (22 as u8); }
    Status::Ok
}

pub fn filter_stage_23(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (23 as u8); }
    Status::Ok
}

pub fn filter_stage_24(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (24 as u8); }
    Status::Ok
}

pub fn filter_stage_25(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (25 as u8); }
    Status::Ok
}

pub fn filter_stage_26(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (26 as u8); }
    Status::Ok
}

pub fn filter_stage_27(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (27 as u8); }
    Status::Ok
}

pub fn filter_stage_28(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (28 as u8); }
    Status::Ok
}

pub fn filter_stage_29(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (29 as u8); }
    Status::Ok
}

pub fn filter_stage_30(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (30 as u8); }
    Status::Ok
}

pub fn filter_stage_31(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (31 as u8); }
    Status::Ok
}

pub fn filter_stage_32(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (32 as u8); }
    Status::Ok
}

pub fn filter_stage_33(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (33 as u8); }
    Status::Ok
}

pub fn filter_stage_34(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (34 as u8); }
    Status::Ok
}

pub fn filter_stage_35(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (35 as u8); }
    Status::Ok
}

pub fn filter_stage_36(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (36 as u8); }
    Status::Ok
}

pub fn filter_stage_37(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (37 as u8); }
    Status::Ok
}

pub fn filter_stage_38(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (38 as u8); }
    Status::Ok
}

pub fn filter_stage_39(input: Span<'_>, out: &mut [u8]) -> Status {
    let n = input.len().min(out.len());
    out[..n].copy_from_slice(&input.data[..n]);
    for b in &mut out[..n] { *b ^= (39 as u8); }
    Status::Ok
}
