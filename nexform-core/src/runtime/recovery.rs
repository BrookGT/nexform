//! Recoverable section replay manager.


use crate::common::types::{has_flag, RecoveryAction, SectionFlags, Span, Status};
use crate::core::arena::Arena;
use crate::core::object_pool::ObjectPool;
use crate::core::state_machine::StateMachine;

pub struct RecoveryManager<'a> {
    arena: &'a mut Arena,
    pool: &'a mut ObjectPool,
    state_machine: &'a mut StateMachine,
    recovered_sections: Vec<u16>,
    recovery_generation: u32,
}

impl<'a> RecoveryManager<'a> {
    pub fn new(arena: &'a mut Arena, pool: &'a mut ObjectPool, state_machine: &'a mut StateMachine) -> Self {
        Self { arena, pool, state_machine, recovered_sections: Vec::new(), recovery_generation: 0 }
    }

    pub fn execute(&mut self, action: RecoveryAction, section_id: u16) -> Status {
        match action {
            RecoveryAction::ReleasePoolSlot => self.release_pool_slot(section_id),
            RecoveryAction::ResetArena => self.reset_arena_region(section_id),
            RecoveryAction::InvalidateCache => { self.state_machine.invalidate_cache(); Status::Ok }
            RecoveryAction::RewindSection => self.invalidate_and_rewind(section_id),
            RecoveryAction::ReplayDecode => self.replay_from_section(section_id),
            _ => Status::Ok,
        }
    }

    pub fn handle_checksum_failure(&mut self, section_id: u16, _payload: Span<'_>) -> Status {
        let Some(section) = self.pool.lookup(section_id) else { return Status::XrefUnresolved; };
        if has_flag(section.descriptor().flags, SectionFlags::Recoverable) {
            self.arena.unpin_slot(section_id);
            let _ = self.release_pool_slot(section_id);
            self.recovered_sections.push(section_id);
            self.recovery_generation = self.recovery_generation.saturating_add(1);
            return Status::Ok;
        }
        Status::ChecksumMismatch
    }

    pub fn replay_from_section(&mut self, section_id: u16) -> Status {
        if self.pool.lookup(section_id).is_none() {
            self.pool.acquire(section_id);
        }
        if let Some(section) = self.pool.lookup_mut(section_id) {
            section.set_active(true);
        }
        self.state_machine.set_active_section(section_id);
        Status::Ok
    }

    fn release_pool_slot(&mut self, section_id: u16) -> Status {
        self.pool.recycle_for_recovery(section_id);
        self.pool.mark_stale(section_id);
        Status::Ok
    }

    fn reset_arena_region(&mut self, section_id: u16) -> Status {
        if self.arena.is_pinned(section_id) { self.arena.unpin_slot(section_id); }
        Status::Ok
    }

    fn invalidate_and_rewind(&mut self, section_id: u16) -> Status {
        self.state_machine.invalidate_cache();
        self.pool.mark_stale(section_id);
        Status::Ok
    }
}
