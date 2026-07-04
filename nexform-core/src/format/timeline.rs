//! Pipeline timeline builder for export and diagnostics.


use crate::common::types::{SectionDescriptor, Status};

#[derive(Debug, Clone, Copy)]
pub enum TimelineEventKind {
    SectionAdded,
    Validation,
    Recovery,
    Annotation,
    Pipeline,
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub kind: TimelineEventKind,
    pub status: Status,
    pub section_id: u16,
    pub detail: alloc::string::String,
    pub timestamp: u32,
}

pub struct TimelineBuilder {
    origin: u32,
    events: Vec<TimelineEvent>,
}

impl Default for TimelineBuilder {
    fn default() -> Self { Self { origin: 0, events: Vec::new() } }
}

impl TimelineBuilder {
    pub fn set_document_origin(&mut self, ts: u32) { self.origin = ts; }
    pub fn events(&self) -> &[TimelineEvent] { &self.events }

    pub fn add_section_event(&mut self, desc: &SectionDescriptor, kind: TimelineEventKind) {
        self.events.push(TimelineEvent {
            kind,
            status: Status::Ok,
            section_id: desc.section_id,
            detail: crate::common::types::section_type_to_string(desc.ty).into(),
            timestamp: self.origin,
        });
    }

    pub fn add_pipeline_event(&mut self, kind: TimelineEventKind, status: Status, section_id: u16, detail: &str) {
        self.events.push(TimelineEvent { kind, status, section_id, detail: detail.into(), timestamp: self.origin });
    }

    pub fn add_annotation_event(&mut self, rec: &crate::format::annotation_parser::AnnotationRecord) {
        self.events.push(TimelineEvent {
            kind: TimelineEventKind::Annotation,
            status: Status::Ok,
            section_id: 0,
            detail: rec.label.clone(),
            timestamp: rec.timestamp,
        });
    }
}
