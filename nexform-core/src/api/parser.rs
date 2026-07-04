//! Top-level NXF parser entry points.


use crate::api::session::Session;
use crate::common::types::{HeaderInfo, Status};
use crate::format::header::HeaderParser;

pub struct Parser {
    session: Session,
}

impl Default for Parser {
    fn default() -> Self { Self::new() }
}

impl Parser {
    pub fn new() -> Self { Self { session: Session::new() } }

    pub fn parse_document(&mut self, data: &[u8]) -> Status {
        if data.is_empty() { return Status::Incomplete; }
        self.session.process_input(crate::common::types::Span::new(data))
    }

    pub fn parse_header_only(&self, data: &[u8], out: &mut HeaderInfo) -> Status {
        if data.len() < 32 { return Status::Incomplete; }
        let mut sections = Vec::new();
        HeaderParser::parse(crate::common::types::Span::new(data), out, &mut sections)
    }

    pub fn decode_payload(&mut self, data: &[u8]) -> Status {
        if data.is_empty() { return Status::Incomplete; }
        let st = self.session.process_partial(crate::common::types::Span::new(data), false);
        if st != Status::Ok { return st; }
        for _ in 0..data.len() / 64 + 1 {
            let _ = self.session.resume_after_recovery();
        }
        self.session.process_partial(crate::common::types::Span::new(data), true)
    }

    pub fn run_state_machine(&mut self, data: &[u8]) -> Status {
        if data.is_empty() { return Status::Incomplete; }
        let st = self.session.process_partial(crate::common::types::Span::new(data), false);
        if st != Status::Ok { return st; }
        for ev in [0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07] {
            if self.session.handle_state_event(ev) != Status::Ok { break; }
        }
        self.session.process_partial(crate::common::types::Span::new(data), true)
    }

    pub fn session(&self) -> &Session { &self.session }
    pub fn session_mut(&mut self) -> &mut Session { &mut self.session }
    pub fn stats(&self) -> &crate::common::types::ParseStats { self.session.stats() }
}

pub fn parse(data: &[u8]) -> Status { Parser::new().parse_document(data) }
pub fn decode(data: &[u8]) -> Status { Parser::new().decode_payload(data) }
pub fn process_state_input(data: &[u8]) -> Status { Parser::new().run_state_machine(data) }
