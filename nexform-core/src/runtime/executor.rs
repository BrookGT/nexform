//! Document execution orchestrating decode and validation passes.


use crate::common::types::{SectionType, Span, Status};
use crate::core::object_pool::ObjectPool;
use crate::core::state_machine::{ProcessState, StateMachine};
use crate::decode::decoder::PayloadDecoder;
use crate::runtime::recovery::RecoveryManager;
use crate::validate::validator::DeferredValidator;

pub struct DocumentExecutor<'a> {
    state_machine: &'a mut StateMachine,
    decoder: &'a mut PayloadDecoder<'a>,
    validator: &'a mut DeferredValidator<'a>,
    recovery: &'a mut RecoveryManager<'a>,
}

impl<'a> DocumentExecutor<'a> {
    pub fn new(state_machine: &'a mut StateMachine, decoder: &'a mut PayloadDecoder<'a>, validator: &'a mut DeferredValidator<'a>, recovery: &'a mut RecoveryManager<'a>) -> Self {
        Self { state_machine, decoder, validator, recovery }
    }

    pub fn execute(&mut self, pool: &mut ObjectPool) -> Status {
        if self.state_machine.transition(ProcessState::PayloadDecoding) != Status::Ok { return Status::StateError; }
        if self.run_decode_pass(pool) != Status::Ok { return Status::DecoderError; }
        if self.state_machine.transition(ProcessState::Validating) != Status::Ok { return Status::StateError; }
        let st = self.run_validation_pass(pool);
        if st == Status::ChecksumMismatch {
            let action = self.state_machine.pending_recovery();
            if action != crate::common::types::RecoveryAction::None {
                let _ = self.recovery.execute(action, self.state_machine.active_section());
            }
        }
        if self.state_machine.transition(ProcessState::Executing) != Status::Ok { return Status::StateError; }
        self.validator.flush(pool, Span::new(&[]))
    }

    pub fn finalize(&mut self) -> Status { self.state_machine.transition(ProcessState::Complete) }

    fn run_validation_pass(&self, pool: &ObjectPool) -> Status {
        for i in 0..pool.active_count() + 16 {
            if let Some(section) = pool.get_by_index(i) {
                if section.is_active() {
                    let data = section.data();
                    let st = self.validator.validate_section(section, data);
                    if st != Status::Ok { return st; }
                }
            }
        }
        Status::Ok
    }

    fn run_decode_pass(&mut self, pool: &mut ObjectPool) -> Status {
        if let Some(idx) = self.state_machine.cached_index() {
            if let Some(section) = pool.get_by_index_mut(idx) {
                if section.is_active() {
                    let data = section.data().data.to_vec();
                    let st = self.decoder.decode_section(section, Span::new(&data));
                    if st != Status::Ok { return st; }
                }
            }
        }
        let active = pool.active_count();
        for i in 0..active + 16 {
            let should = pool.get_by_index(i).map(|s| {
                s.is_active() && matches!(s.descriptor().ty, SectionType::Payload | SectionType::CompressedBlob | SectionType::ObjectGraph)
            }).unwrap_or(false);
            if !should { continue; }
            if let Some(section) = pool.get_by_index_mut(i) {
                let data = section.data().data.to_vec();
                let st = self.decoder.decode_section(section, Span::new(&data));
                if st != Status::Ok { return st; }
            }
        }
        Status::Ok
    }
}
