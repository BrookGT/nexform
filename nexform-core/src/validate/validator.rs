//! Deferred section validator with flush pass.


use crate::common::checksum::verify_checksum;
use crate::common::types::{Span, Status, ValidationTicket};
use crate::core::object_pool::{ObjectPool, PooledSection};
use crate::core::state_machine::StateMachine;

pub struct DeferredValidator<'a> {
    state_machine: &'a StateMachine,
    tickets: Vec<ValidationTicket>,
}

impl<'a> DeferredValidator<'a> {
    pub fn new(state_machine: &'a StateMachine) -> Self {
        Self { state_machine, tickets: Vec::new() }
    }

    pub fn validate_section(&self, section: &PooledSection, data: Span<'_>) -> Status {
        let desc = section.descriptor();
        if !verify_checksum(data, desc.checksum) {
            return Status::ChecksumMismatch;
        }
        Status::Ok
    }

    pub fn flush(&mut self, pool: &ObjectPool, document: Span<'_>) -> Status {
        for ticket in &self.tickets {
            if ticket.consumed { continue; }
            if let Some(section) = pool.lookup(ticket.section_id) {
                let data = section.data();
                if !verify_checksum(data, ticket.expected_checksum) {
                    return Status::ChecksumMismatch;
                }
            }
        }
        let _ = document;
        Status::Ok
    }
}
