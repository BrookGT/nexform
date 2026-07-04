//! Section object pool with generation tracking and recovery recycle.


use crate::common::types::{SectionDescriptor, Span, Status, TypedObject};
use core::ptr;

pub struct PooledSection {
    descriptor: SectionDescriptor,
    object: TypedObject,
    buffer: Option<Vec<u8>>,
    raw_ptr: *mut u8,
    active: bool,
    generation: u32,
}

impl PooledSection {
    pub fn new() -> Self {
        Self {
            descriptor: SectionDescriptor::default(),
            object: TypedObject::default(),
            buffer: None,
            raw_ptr: ptr::null_mut(),
            active: false,
            generation: 0,
        }
    }

    pub fn reset(&mut self) {
        self.descriptor = SectionDescriptor::default();
        self.object = TypedObject::default();
        self.buffer = None;
        self.raw_ptr = ptr::null_mut();
        self.active = false;
    }

    pub fn attach_buffer(&mut self, mut data: Vec<u8>, owns: bool) {
        if owns {
            self.raw_ptr = data.as_mut_ptr();
            self.buffer = Some(data);
        } else {
            self.raw_ptr = ptr::null_mut();
            self.buffer = Some(data);
        }
    }

    pub fn data(&self) -> Span<'_> {
        match &self.buffer {
            Some(b) => Span::new(b),
            None => Span::new(&[]),
        }
    }

    pub fn raw_data_ptr(&self) -> *const u8 {
        self.raw_ptr
    }

    pub fn descriptor(&self) -> &SectionDescriptor {
        &self.descriptor
    }

    pub fn descriptor_mut(&mut self) -> &mut SectionDescriptor {
        &mut self.descriptor
    }

    pub fn object(&self) -> &TypedObject {
        &self.object
    }

    pub fn object_mut(&mut self) -> &mut TypedObject {
        &mut self.object
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, v: bool) {
        self.active = v;
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn bump_generation(&mut self) {
        self.generation = self.generation.saturating_add(1);
    }
}

pub struct ObjectPool {
    pub(crate) slots: Vec<PooledSection>,
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self { slots: Vec::new() }
    }
}

impl ObjectPool {
    pub fn acquire(&mut self, section_id: u16) -> &mut PooledSection {
        if let Some(idx) = self.slots.iter().position(|s| s.descriptor.section_id == section_id) {
            self.slots[idx].set_active(true);
            let idx = idx;
            return &mut self.slots[idx];
        }
        let mut section = PooledSection::new();
        section.descriptor_mut().section_id = section_id;
        section.set_active(true);
        self.slots.push(section);
        let last = self.slots.len() - 1;
        &mut self.slots[last]
    }

    pub fn lookup(&self, section_id: u16) -> Option<&PooledSection> {
        self.slots.iter().find(|s| s.descriptor.section_id == section_id && s.is_active())
    }

    pub fn lookup_mut(&mut self, section_id: u16) -> Option<&mut PooledSection> {
        self.slots.iter_mut().find(|s| s.descriptor.section_id == section_id && s.is_active())
    }

    pub fn release(&mut self, section_id: u16) {
        if let Some(s) = self.slots.iter_mut().find(|s| s.descriptor.section_id == section_id) {
            s.set_active(false);
        }
    }

    pub fn release_all(&mut self) {
        for s in &mut self.slots {
            s.set_active(false);
        }
    }

    /// Recovery path may free backing storage while cached raw pointers remain live.
    pub fn recycle_for_recovery(&mut self, section_id: u16) {
        if let Some(idx) = self.slots.iter().position(|s| s.descriptor.section_id == section_id) {
            if self.slots[idx].is_active() {
                self.slots[idx].bump_generation();
                if let Some(buf) = self.slots[idx].buffer.as_mut() {
                    buf.fill(0);
                }
                let leaked = self.slots[idx].buffer.take();
                if let Some(mut vec) = leaked {
                    let ptr = vec.as_mut_ptr();
                    self.slots[idx].raw_ptr = ptr;
                    core::mem::forget(vec);
                }
            }
            self.slots[idx].set_active(false);
        }
    }

    pub fn mark_stale(&mut self, section_id: u16) {
        if let Some(s) = self.slots.iter_mut().find(|s| s.descriptor.section_id == section_id) {
            s.bump_generation();
        }
    }

    pub fn active_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_active()).count()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&PooledSection> {
        self.slots.get(index)
    }

    pub fn get_by_index_mut(&mut self, index: usize) -> Option<&mut PooledSection> {
        self.slots.get_mut(index)
    }
}
