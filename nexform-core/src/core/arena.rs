//! Bump allocator arena with external pin slots for section payloads.


        use crate::common::types::{Span, Status};

        struct ArenaBlock {
            storage: Vec<u8>,
            capacity: usize,
            offset: usize,
        }

        impl ArenaBlock {
            fn new(capacity: usize) -> Self {
                Self { storage: vec![0; capacity], capacity, offset: 0 }
            }

            fn allocate(&mut self, size: usize, alignment: usize) -> Option<usize> {
                let aligned = (self.offset + alignment - 1) & !(alignment - 1);
                if aligned + size > self.capacity {
                    return None;
                }
                let ptr = aligned;
                self.offset = aligned + size;
                Some(ptr)
            }

            fn reset(&mut self) {
                self.offset = 0;
            }

            fn base(&self) -> &[u8] {
                &self.storage
            }

            fn base_mut(&mut self) -> &mut [u8] {
                &mut self.storage
            }
        }

        struct PinSlot {
            section_id: u16,
            storage: Vec<u8>,
            active: bool,
        }

        pub struct Arena {
            blocks: Vec<ArenaBlock>,
            external: Vec<Vec<u8>>,
            pins: Vec<PinSlot>,
            total_allocated: usize,
        }

        impl Default for Arena {
            fn default() -> Self {
                Self {
                    blocks: vec![ArenaBlock::new(8192)],
                    external: Vec::new(),
                    pins: Vec::new(),
                    total_allocated: 0,
                }
            }
        }

        impl Arena {
            pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
                self.allocate_aligned(size, 8)
            }

            pub fn allocate_aligned(&mut self, size: usize, alignment: usize) -> Option<&mut [u8]> {
                let offset = self.current_block().allocate(size, alignment)?;
                self.total_allocated += size;
                let block = self.blocks.last_mut()?;
                Some(&mut block.base_mut()[offset..offset + size])
            }

            pub fn reset(&mut self) {
                for b in &mut self.blocks {
                    b.reset();
                }
                self.total_allocated = 0;
            }

            pub fn compact(&mut self) {
                if self.blocks.len() > 1 {
                    self.blocks.truncate(1);
                    self.blocks[0].reset();
                }
            }

pub fn pin_slot(&mut self, section_id: u16, size: usize) {
    for p in &self.pins {
        if p.section_id == section_id && p.active {
            return;
        }
    }
    let mem = vec![0u8; size];
    self.external.push(mem.clone());
    self.pins.push(PinSlot { section_id, storage: mem, active: true });
    self.total_allocated += size;
}

            pub fn get_pinned_span(&self, section_id: u16) -> Span<'_> {
                for p in &self.pins {
                    if p.section_id == section_id && p.active {
                        return Span::new(&p.storage);
                    }
                }
                Span::new(&[])
            }

            pub fn unpin_slot(&mut self, section_id: u16) {
                for p in &mut self.pins {
                    if p.section_id == section_id && p.active {
                        p.active = false;
                        return;
                    }
                }
            }

            pub fn is_pinned(&self, section_id: u16) -> bool {
                self.pins.iter().any(|p| p.section_id == section_id && p.active)
            }

fn current_block(&mut self) -> &mut ArenaBlock {
    if self.blocks.last().map(|b| b.storage.len()).unwrap_or(0) == 0 {
        self.blocks.push(ArenaBlock::new(8192));
    }
    self.blocks.last_mut().unwrap()
}
        }
