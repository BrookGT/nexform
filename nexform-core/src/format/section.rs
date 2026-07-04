//! Section loader copying payload bytes into arena and object pool.


use crate::common::types::{SectionDescriptor, Span, Status};
use crate::core::arena::Arena;
use crate::core::object_pool::ObjectPool;

pub struct SectionLoader<'a> {
    arena: &'a mut Arena,
    pool: &'a mut ObjectPool,
}

impl<'a> SectionLoader<'a> {
    pub fn new(arena: &'a mut Arena, pool: &'a mut ObjectPool) -> Self {
        Self { arena, pool }
    }

    pub fn load_all(&mut self, input: Span<'_>, descriptors: &[SectionDescriptor]) -> Status {
        for desc in descriptors {
            if desc.offset as usize + desc.length as usize > input.len() {
                if crate::common::types::has_flag(desc.flags, crate::common::types::SectionFlags::Optional) {
                    continue;
                }
                return Status::SectionOutOfRange;
            }
            let slice = &input.data[desc.offset as usize..desc.offset as usize + desc.length as usize];
            let section = self.pool.acquire(desc.section_id);
            *section.descriptor_mut() = *desc;
            section.attach_buffer(slice.to_vec(), true);
        }
        Status::Ok
    }
}
