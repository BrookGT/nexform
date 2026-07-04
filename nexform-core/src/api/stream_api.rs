//! Incremental streaming API for large NXF documents.


use crate::api::session::Session;
use crate::common::types::{Span, Status};
use crate::runtime::stream_reader::StreamReader;

pub struct StreamSession {
    session: Session,
    reader: StreamReader,
}

impl Default for StreamSession {
    fn default() -> Self { Self::new() }
}

impl StreamSession {
    pub fn new() -> Self {
        Self { session: Session::new(), reader: StreamReader::default() }
    }

    pub fn feed(&mut self, chunk: &[u8]) -> Status {
        let span = Span::new(chunk);
        let st = self.reader.feed(span);
        if st == Status::Ok {
            self.session.process_partial(span, false)
        } else {
            st
        }
    }

    pub fn finalize(&mut self, chunk: &[u8]) -> Status {
        self.session.process_partial(Span::new(chunk), true)
    }

    pub fn inner(&self) -> &Session { &self.session }
}
