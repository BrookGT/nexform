//! Incremental stream reader for large NXF documents.

use crate::common::types::{Span, Status};
use crate::format::header::HeaderParser;
use crate::common::types::{HeaderInfo, SectionDescriptor};

pub struct StreamReader { header: HeaderInfo, sections: Vec<SectionDescriptor>, consumed: usize }

impl Default for StreamReader { fn default() -> Self { Self { header: HeaderInfo::default(), sections: Vec::new(), consumed: 0 } } }

impl StreamReader {
    pub fn feed(&mut self, chunk: Span<'_>) -> Status {
        let mut header = HeaderInfo::default();
        let mut sections = Vec::new();
        let st = HeaderParser::parse(chunk, &mut header, &mut sections);
        if st == Status::Ok { self.header = header; self.sections = sections; self.consumed = chunk.len(); }
        st
    }
    pub fn sections(&self) -> &[SectionDescriptor] { &self.sections }
}

pub fn stream_window_00(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 0).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_01(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 1).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_02(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 2).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_03(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 3).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_04(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 4).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_05(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 5).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_06(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 6).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_07(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 7).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_08(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 8).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_09(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 9).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_10(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 10).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_11(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 11).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_12(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 12).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_13(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 13).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_14(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 14).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_15(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 15).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_16(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 16).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_17(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 17).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_18(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 18).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_19(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 19).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_20(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 20).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_21(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 21).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_22(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 22).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_23(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 23).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_24(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 24).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_25(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 25).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_26(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 26).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_27(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 27).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_28(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 28).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_29(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 29).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_30(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 30).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_31(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 31).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_32(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 32).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_33(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 33).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_34(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 34).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_35(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 35).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_36(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 36).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_37(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 37).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_38(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 38).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_39(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 39).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_40(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 40).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_41(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 41).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_42(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 42).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_43(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 43).min(data.len());
    Span::new(&data.data[start..end])
}

pub fn stream_window_44(data: Span<'_>, offset: usize) -> Span<'_> {
    let start = offset.min(data.len());
    let end = (start + 64 + 44).min(data.len());
    Span::new(&data.data[start..end])
}
