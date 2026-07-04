//! Compression codec dispatch for NXF compressed blobs.


use crate::common::types::{Span, Status};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionCodec {
    None = 0,
    Rle = 1,
    LzLite = 2,
    Delta = 3,
}

pub fn decompress(codec: CompressionCodec, input: Span<'_>, out: &mut [u8]) -> Status {
    match codec {
        CompressionCodec::None => {
            if input.len() > out.len() { return Status::AllocationFailed; }
            out[..input.len()].copy_from_slice(input.data);
            Status::Ok
        }
        CompressionCodec::Rle => decompress_rle(input, out),
        CompressionCodec::LzLite => decompress_lzlite(input, out),
        CompressionCodec::Delta => decompress_delta(input, out),
    }
}

fn decompress_rle(input: Span<'_>, out: &mut [u8]) -> Status {
    let mut o = 0usize;
    let mut i = 0usize;
    while i + 2 <= input.len() && o < out.len() {
        let count = input.data[i] as usize;
        let value = input.data[i + 1];
        for _ in 0..count {
            if o >= out.len() { break; }
            out[o] = value;
            o += 1;
        }
        i += 2;
    }
    Status::Ok
}

fn decompress_lzlite(input: Span<'_>, out: &mut [u8]) -> Status {
    if input.len() > out.len() { return Status::AllocationFailed; }
    out[..input.len()].copy_from_slice(input.data);
    Status::Ok
}

fn decompress_delta(input: Span<'_>, out: &mut [u8]) -> Status {
    if input.is_empty() { return Status::Ok; }
    out[0] = input.data[0];
    for i in 1..input.len().min(out.len()) {
        out[i] = out[i - 1].wrapping_add(input.data[i]);
    }
    Status::Ok
}
