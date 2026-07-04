//! Document parse state machine with recovery transitions.


use crate::common::types::{RecoveryAction, Status};
use crate::core::object_pool::PooledSection;
use crate::format::spec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Idle,
    HeaderParsed,
    SectionsIndexed,
    MetadataLoaded,
    XrefResolved,
    PayloadDecoding,
    Validating,
    Recovering,
    Executing,
    Complete,
    Failed,
}

struct Transition {
    from: ProcessState,
    to: ProcessState,
    flag_mask: u32,
    recovery: RecoveryAction,
}

pub struct StateMachine {
    current: ProcessState,
    history: Vec<ProcessState>,
    generation: u32,
    active_section: u16,
    interpret_mode: crate::common::types::InterpretMode,
    cached_index: Option<usize>,
    cached_generation: u32,
    pending_recovery: RecoveryAction,
    transitions: Vec<Transition>,
    pool_generation: u32,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self {
            current: ProcessState::Idle,
            history: Vec::new(),
            generation: 0,
            active_section: 0,
            interpret_mode: crate::common::types::InterpretMode::Raw,
            cached_index: None,
            cached_generation: 0,
            pending_recovery: RecoveryAction::None,
            pool_generation: 0,
            transitions: vec![
                Transition { from: ProcessState::Idle, to: ProcessState::HeaderParsed, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::HeaderParsed, to: ProcessState::SectionsIndexed, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::SectionsIndexed, to: ProcessState::MetadataLoaded, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::MetadataLoaded, to: ProcessState::XrefResolved, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::XrefResolved, to: ProcessState::PayloadDecoding, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::PayloadDecoding, to: ProcessState::Validating, flag_mask: 0, recovery: RecoveryAction::RewindSection },
                Transition { from: ProcessState::Validating, to: ProcessState::Executing, flag_mask: 0, recovery: RecoveryAction::InvalidateCache },
                Transition { from: ProcessState::Validating, to: ProcessState::Recovering, flag_mask: u32::from(spec::DOCUMENT_FLAG_RECOVERABLE), recovery: RecoveryAction::ReleasePoolSlot },
                Transition { from: ProcessState::Recovering, to: ProcessState::PayloadDecoding, flag_mask: 0, recovery: RecoveryAction::ReplayDecode },
                Transition { from: ProcessState::Executing, to: ProcessState::Complete, flag_mask: 0, recovery: RecoveryAction::None },
                Transition { from: ProcessState::PayloadDecoding, to: ProcessState::Failed, flag_mask: 0, recovery: RecoveryAction::ResetArena },
            ],
        }
    }
}

impl StateMachine {
    pub fn current(&self) -> ProcessState { self.current }
    pub fn active_section(&self) -> u16 { self.active_section }
    pub fn interpretation(&self) -> crate::common::types::InterpretMode { self.interpret_mode }
    pub fn pending_recovery(&self) -> RecoveryAction { self.pending_recovery }

    pub fn set_active_section(&mut self, id: u16) { self.active_section = id; }
    pub fn set_interpret_mode(&mut self, mode: crate::common::types::InterpretMode) { self.interpret_mode = mode; }

    pub fn transition(&mut self, target: ProcessState) -> Status {
        if !self.can_transition(target) { return Status::StateError; }
        self.on_leave(self.current);
        self.history.push(self.current);
        self.current = target;
        self.generation = self.generation.saturating_add(1);
        self.on_enter(target);
        Status::Ok
    }

    pub fn handle_event(&mut self, code: u8) -> Status {
        match code {
            0x01 => self.transition(ProcessState::HeaderParsed),
            0x02 => self.transition(ProcessState::SectionsIndexed),
            0x03 => self.transition(ProcessState::MetadataLoaded),
            0x04 => self.transition(ProcessState::XrefResolved),
            0x05 => self.transition(ProcessState::PayloadDecoding),
            0x06 => self.transition(ProcessState::Validating),
            0x07 => self.transition(ProcessState::Executing),
            0xFE => { self.enter_recovery(RecoveryAction::ReleasePoolSlot); self.transition(ProcessState::Recovering) }
            _ => Status::StateError,
        }
    }

    pub fn can_transition(&self, target: ProcessState) -> bool {
        self.transitions.iter().any(|t| t.from == self.current && t.to == target)
    }

    pub fn cache_object_index(&mut self, index: usize, generation: u32) {
        self.cached_index = Some(index);
        self.cached_generation = generation;
    }

    pub fn cached_index(&self) -> Option<usize> { self.cached_index }

    pub fn cached_generation(&self) -> u32 { self.cached_generation }

    pub fn invalidate_cache(&mut self) {
        self.cached_index = None;
    }

    pub fn enter_recovery(&mut self, action: RecoveryAction) {
        self.pending_recovery = action;
    }

    pub fn clear_recovery(&mut self) {
        self.pending_recovery = RecoveryAction::None;
    }

    fn validate_transition(&self, from: ProcessState, to: ProcessState) -> bool {
        self.transitions.iter().any(|t| t.from == from && t.to == to)
    }

    fn on_enter(&mut self, state: ProcessState) {
        if state == ProcessState::Complete {
            self.invalidate_cache();
        }
    }

    fn on_leave(&mut self, state: ProcessState) {
        if state == ProcessState::Recovering {
            self.clear_recovery();
        }
    }
}
