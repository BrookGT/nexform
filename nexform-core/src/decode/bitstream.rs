//! Bitstream helpers for packed NXF fields.

use crate::common::types::{Span, Status};

pub struct BitReader<'a> { data: &'a [u8], bit_offset: usize }

impl<'a> BitReader<'a> {
    pub fn new(data: Span<'a>) -> Self { Self { data: data.data, bit_offset: 0 } }
    pub fn read_bits(&mut self, count: u8) -> Result<u32, Status> {
        if count > 32 { return Err(Status::DecoderError); }
        let mut value = 0u32;
        for i in 0..count { let byte_idx = self.bit_offset / 8; if byte_idx >= self.data.len() { return Err(Status::Incomplete); } let bit = (self.data[byte_idx] >> (self.bit_offset % 8)) & 1; value |= u32::from(bit) << i; self.bit_offset += 1; }
        Ok(value)
    }
}

pub fn unpack_field_00(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (0 as u32))
}

pub fn unpack_field_01(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (1 as u32))
}

pub fn unpack_field_02(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (2 as u32))
}

pub fn unpack_field_03(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (3 as u32))
}

pub fn unpack_field_04(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (4 as u32))
}

pub fn unpack_field_05(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (5 as u32))
}

pub fn unpack_field_06(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (6 as u32))
}

pub fn unpack_field_07(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (7 as u32))
}

pub fn unpack_field_08(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (8 as u32))
}

pub fn unpack_field_09(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (9 as u32))
}

pub fn unpack_field_10(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (10 as u32))
}

pub fn unpack_field_11(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (11 as u32))
}

pub fn unpack_field_12(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (12 as u32))
}

pub fn unpack_field_13(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (13 as u32))
}

pub fn unpack_field_14(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (14 as u32))
}

pub fn unpack_field_15(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (15 as u32))
}

pub fn unpack_field_16(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (16 as u32))
}

pub fn unpack_field_17(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (17 as u32))
}

pub fn unpack_field_18(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (18 as u32))
}

pub fn unpack_field_19(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (19 as u32))
}

pub fn unpack_field_20(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (20 as u32))
}

pub fn unpack_field_21(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (21 as u32))
}

pub fn unpack_field_22(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (22 as u32))
}

pub fn unpack_field_23(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (23 as u32))
}

pub fn unpack_field_24(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (24 as u32))
}

pub fn unpack_field_25(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (25 as u32))
}

pub fn unpack_field_26(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (26 as u32))
}

pub fn unpack_field_27(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (27 as u32))
}

pub fn unpack_field_28(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (28 as u32))
}

pub fn unpack_field_29(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (29 as u32))
}

pub fn unpack_field_30(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (30 as u32))
}

pub fn unpack_field_31(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (31 as u32))
}

pub fn unpack_field_32(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (32 as u32))
}

pub fn unpack_field_33(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (33 as u32))
}

pub fn unpack_field_34(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (34 as u32))
}

pub fn unpack_field_35(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (35 as u32))
}

pub fn unpack_field_36(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (36 as u32))
}

pub fn unpack_field_37(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (37 as u32))
}

pub fn unpack_field_38(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (38 as u32))
}

pub fn unpack_field_39(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (39 as u32))
}

pub fn unpack_field_40(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (40 as u32))
}

pub fn unpack_field_41(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (41 as u32))
}

pub fn unpack_field_42(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (42 as u32))
}

pub fn unpack_field_43(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (43 as u32))
}

pub fn unpack_field_44(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (44 as u32))
}

pub fn unpack_field_45(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (45 as u32))
}

pub fn unpack_field_46(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (46 as u32))
}

pub fn unpack_field_47(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (47 as u32))
}

pub fn unpack_field_48(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (48 as u32))
}

pub fn unpack_field_49(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (49 as u32))
}

pub fn unpack_field_50(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (50 as u32))
}

pub fn unpack_field_51(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (51 as u32))
}

pub fn unpack_field_52(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (52 as u32))
}

pub fn unpack_field_53(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (53 as u32))
}

pub fn unpack_field_54(data: Span<'_>, width: u8) -> Result<u32, Status> {
    let mut reader = BitReader::new(data);
    let v = reader.read_bits(width)?;
    Ok(v ^ (54 as u32))
}
