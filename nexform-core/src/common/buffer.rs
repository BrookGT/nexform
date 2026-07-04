//! Byte readers/writers and growable buffers for NXF parsing.


use core::marker::PhantomData;

use crate::common::types::{SectionType, Span, Status};

pub struct ByteReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(span: Span<'a>) -> Self {
        Self { data: span.data, offset: 0 }
    }

    pub fn reset(&mut self, span: Span<'a>) {
        self.data = span.data;
        self.offset = 0;
    }

    pub fn has_bytes(&self, n: usize) -> bool {
        self.offset + n <= self.data.len()
    }

    pub fn read_u8(&mut self) -> Result<u8, Status> {
        if !self.has_bytes(1) {
            return Err(Status::Incomplete);
        }
        let v = self.data[self.offset];
        self.offset += 1;
        Ok(v)
    }

    pub fn read_u16_le(&mut self) -> Result<u16, Status> {
        if !self.has_bytes(2) {
            return Err(Status::Incomplete);
        }
        let v = u16::from_le_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;
        Ok(v)
    }

    pub fn read_u32_le(&mut self) -> Result<u32, Status> {
        if !self.has_bytes(4) {
            return Err(Status::Incomplete);
        }
        let b = &self.data[self.offset..self.offset + 4];
        self.offset += 4;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn read_u64_le(&mut self) -> Result<u64, Status> {
        if !self.has_bytes(8) {
            return Err(Status::Incomplete);
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&self.data[self.offset..self.offset + 8]);
        self.offset += 8;
        Ok(u64::from_le_bytes(arr))
    }

    pub fn read_bytes(&mut self, out: &mut [u8]) -> Result<(), Status> {
        if !self.has_bytes(out.len()) {
            return Err(Status::Incomplete);
        }
        out.copy_from_slice(&self.data[self.offset..self.offset + out.len()]);
        self.offset += out.len();
        Ok(())
    }

    pub fn skip(&mut self, n: usize) -> Result<(), Status> {
        if !self.has_bytes(n) {
            return Err(Status::Incomplete);
        }
        self.offset += n;
        Ok(())
    }

    pub fn peek_u32_le(&self) -> Result<u32, Status> {
        if !self.has_bytes(4) {
            return Err(Status::Incomplete);
        }
        Ok(u32::from_le_bytes([
            self.data[self.offset],
            self.data[self.offset + 1],
            self.data[self.offset + 2],
            self.data[self.offset + 3],
        ]))
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }

    pub fn remaining_span(&self) -> Span<'a> {
        Span::new(&self.data[self.offset..])
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct ByteWriter<'a> {
    data_ptr: *mut u8,
    capacity: usize,
    offset: usize,
    growable: Option<*mut GrowableBuffer>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a> ByteWriter<'a> {
    fn data_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data_ptr, self.capacity) }
    }

    pub fn new(span: MutableSpan<'a>) -> Self {
        Self {
            data_ptr: span.data.as_mut_ptr(),
            capacity: span.capacity,
            offset: span.size,
            growable: None,
            _marker: PhantomData,
        }
    }

    pub fn bind(buffer: &mut GrowableBuffer) -> ByteWriter<'_> {
        let ptr = buffer as *mut GrowableBuffer;
        unsafe {
            ByteWriter {
                data_ptr: (*ptr).raw_mut_ptr(),
                capacity: (*ptr).capacity(),
                offset: (*ptr).size(),
                growable: Some(ptr),
                _marker: PhantomData,
            }
        }
    }

    fn ensure_capacity(&mut self, n: usize) -> Status {
        if self.offset + n <= self.capacity {
            return Status::Ok;
        }
        if let Some(ptr) = self.growable {
            unsafe {
                if (*ptr).reserve(self.offset + n) != Status::Ok {
                    return Status::AllocationFailed;
                }
                self.capacity = (*ptr).capacity();
                self.data_ptr = (*ptr).raw_mut_ptr();
            }
        } else {
            return Status::AllocationFailed;
        }
        Status::Ok
    }

    pub fn write_u8(&mut self, value: u8) -> Status {
        if self.ensure_capacity(1) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        unsafe { *self.data_ptr.add(offset) = value; }
        self.offset += 1;
        self.sync_growable()
    }

    pub fn write_u16_le(&mut self, value: u16) -> Status {
        if self.ensure_capacity(2) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        let bytes = value.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.data_ptr.add(offset), 2);
        }
        self.offset += 2;
        self.sync_growable()
    }

    pub fn write_u32_le(&mut self, value: u32) -> Status {
        if self.ensure_capacity(4) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        let bytes = value.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.data_ptr.add(offset), 4);
        }
        self.offset += 4;
        self.sync_growable()
    }

    pub fn write_u64_le(&mut self, value: u64) -> Status {
        if self.ensure_capacity(8) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        let bytes = value.to_le_bytes();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.data_ptr.add(offset), 8);
        }
        self.offset += 8;
        self.sync_growable()
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Status {
        if bytes.is_empty() {
            return Status::Ok;
        }
        if self.ensure_capacity(bytes.len()) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.data_ptr.add(offset), bytes.len());
        }
        self.offset += bytes.len();
        self.sync_growable()
    }

    pub fn write_span(&mut self, span: Span<'_>) -> Status {
        self.write_bytes(span.data)
    }

    pub fn write_zeroes(&mut self, n: usize) -> Status {
        if self.ensure_capacity(n) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        unsafe {
            core::ptr::write_bytes(self.data_ptr.add(offset), 0, n);
        }
        self.offset += n;
        self.sync_growable()
    }

    pub fn seek(&mut self, offset: usize) -> Status {
        if offset > self.capacity {
            return Status::SectionOutOfRange;
        }
        self.offset = offset;
        self.sync_growable()
    }

    pub fn pad_to_alignment(&mut self, alignment: u8, pad_byte: u8) -> Status {
        if alignment <= 1 {
            return Status::Ok;
        }
        let rem = self.offset % alignment as usize;
        if rem == 0 {
            return Status::Ok;
        }
        let pad = alignment as usize - rem;
        if self.ensure_capacity(pad) != Status::Ok { return Status::AllocationFailed; }
        let offset = self.offset;
        unsafe {
            core::ptr::write_bytes(self.data_ptr.add(offset), pad_byte, pad);
        }
        self.offset += pad;
        self.sync_growable()
    }

    fn sync_growable(&mut self) -> Status {
        if let Some(ptr) = self.growable {
            unsafe {
                if (*ptr).resize(self.offset) != Status::Ok {
                    return Status::AllocationFailed;
                }
            }
        }
        Status::Ok
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

pub struct MutableSpan<'a> {
    pub data: &'a mut [u8],
    pub size: usize,
    pub capacity: usize,
}

pub struct GrowableBuffer {
    storage: Vec<u8>,
    capacity: usize,
    size: usize,
}

impl Default for GrowableBuffer {
    fn default() -> Self {
        Self::with_capacity(64)
    }
}

impl GrowableBuffer {
    pub fn with_capacity(initial: usize) -> Self {
        let cap = initial.max(64);
        Self {
            storage: vec![0; cap],
            capacity: cap,
            size: 0,
        }
    }

    pub fn reserve(&mut self, capacity: usize) -> Status {
        if capacity <= self.capacity {
            return Status::Ok;
        }
        self.storage.resize(capacity, 0);
        self.capacity = capacity;
        Status::Ok
    }

    pub fn append(&mut self, data: &[u8]) -> Status {
        if self.size + data.len() > self.capacity {
            let new_cap = (self.capacity * 2).max(self.size + data.len());
            if self.reserve(new_cap) != Status::Ok {
                return Status::AllocationFailed;
            }
        }
        self.storage[self.size..self.size + data.len()].copy_from_slice(data);
        self.size += data.len();
        Status::Ok
    }

    pub fn append_u32_le(&mut self, value: u32) -> Status {
        self.append(&value.to_le_bytes())
    }

    pub fn resize(&mut self, new_size: usize) -> Status {
        if new_size > self.capacity {
            if self.reserve(new_size) != Status::Ok {
                return Status::AllocationFailed;
            }
        }
        self.size = new_size;
        Status::Ok
    }

    pub fn shrink_to_fit(&mut self) {
        self.storage.truncate(self.size);
        self.capacity = self.size;
    }

    pub fn data(&self) -> &[u8] {
        &self.storage[..self.size]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.storage[..self.capacity]
    }

    pub(crate) fn raw_mut_ptr(&mut self) -> *mut u8 {
        self.storage.as_mut_ptr()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn as_span(&self) -> Span<'_> {
        Span::new(self.data())
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}

pub fn write_section_header(
    writer: &mut ByteWriter<'_>,
    ty: SectionType,
    length: u32,
    section_id: u16,
) -> Status {
    if writer.write_u8(ty as u8) != Status::Ok { return Status::InternalError; }
    if writer.write_u16_le(section_id) != Status::Ok { return Status::InternalError; }
    writer.write_u32_le(length)
}

pub struct SlidingWindow {
    window: Vec<u8>,
    window_size: usize,
    head: usize,
    filled: usize,
}

impl SlidingWindow {
    pub fn new(window_size: usize) -> Self {
        Self {
            window: vec![0; window_size],
            window_size,
            head: 0,
            filled: 0,
        }
    }

    pub fn push(&mut self, data: &[u8]) -> Status {
        for &b in data {
            self.window[self.head] = b;
            self.head = (self.head + 1) % self.window_size;
            if self.filled < self.window_size {
                self.filled += 1;
            }
        }
        Status::Ok
    }

    pub fn view(&self) -> Span<'_> {
        if self.filled == 0 {
            return Span::new(&[]);
        }
        let start = (self.head + self.window_size - self.filled) % self.window_size;
        if start + self.filled <= self.window_size {
            return Span::new(&self.window[start..start + self.filled]);
        }
        Span::new(&self.window[..self.filled])
    }

    pub fn advance(&mut self, n: usize) {
        if n >= self.filled {
            self.filled = 0;
            self.head = 0;
        } else {
            self.filled -= n;
        }
    }
}

#[derive(Clone, Copy)]
struct ViewEntry {
    id: u16,
    offset: usize,
    len: usize,
}

pub struct BufferViewCache {
    entries: Vec<ViewEntry>,
    document: Option<Vec<u8>>,
}

impl Default for BufferViewCache {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            document: None,
        }
    }
}

impl BufferViewCache {
    pub fn bind_document(&mut self, doc: Vec<u8>) {
        self.document = Some(doc);
    }

    pub fn store(&mut self, section_id: u16, offset: usize, len: usize) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.id == section_id) {
            e.offset = offset;
            e.len = len;
            return;
        }
        self.entries.push(ViewEntry {
            id: section_id,
            offset,
            len,
        });
    }

    pub fn lookup(&self, section_id: u16) -> Span<'_> {
        let Some(doc) = self.document.as_ref() else {
            return Span::new(&[]);
        };
        for e in &self.entries {
            if e.id == section_id && e.offset + e.len <= doc.len() {
                return Span::new(&doc[e.offset..e.offset + e.len]);
            }
        }
        Span::new(&[])
    }

    pub fn invalidate(&mut self, section_id: u16) {
        self.entries.retain(|e| e.id != section_id);
    }

    pub fn invalidate_all(&mut self) {
        self.entries.clear();
    }
}
