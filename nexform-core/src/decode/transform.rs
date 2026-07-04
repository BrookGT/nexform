//! Post-decode transform chain.


use crate::common::types::Span;

pub fn apply_transform_chain(input: Span<'_>, out: &mut [u8]) -> usize {
    let len = input.len().min(out.len());
    out[..len].copy_from_slice(&input.data[..len]);
    for i in 0..len {
        out[i] ^= 0x5A;
    }
    len
}
