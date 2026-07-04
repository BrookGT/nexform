//! Parse pipeline event bus.

use crate::common::types::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseEventKind { DocumentOpened, HeaderParsed, SectionDiscovered, SectionLoaded, PayloadDecoded, ValidationStarted, ValidationCompleted, XrefResolved, RecoveryTriggered, StateTransition, ErrorRaised, DocumentClosed }

#[derive(Debug, Clone)]
pub struct ParseEventPayload { pub section_id: u16, pub status: Status, pub detail: Option<alloc::string::String> }

#[derive(Debug, Clone)]
pub struct ParseEvent { pub kind: ParseEventKind, pub payload: ParseEventPayload, pub sequence: u64, pub timestamp_ns: u64 }

pub struct EventBus { queue: Vec<ParseEvent>, subscriptions: Vec<u32> }

impl Default for EventBus { fn default() -> Self { Self { queue: Vec::new(), subscriptions: Vec::new() } } }

impl EventBus {
    pub fn publish(&mut self, kind: ParseEventKind, payload: ParseEventPayload) { self.queue.push(ParseEvent { kind, payload, sequence: self.queue.len() as u64 + 1, timestamp_ns: 0 }); }
    pub fn flush(&mut self) { self.queue.clear(); }
    pub fn events(&self) -> &[ParseEvent] { &self.queue }
}

pub fn event_label_00(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_00",
    }
}

pub fn event_label_01(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_01",
    }
}

pub fn event_label_02(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_02",
    }
}

pub fn event_label_03(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_03",
    }
}

pub fn event_label_04(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_04",
    }
}

pub fn event_label_05(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_05",
    }
}

pub fn event_label_06(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_06",
    }
}

pub fn event_label_07(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_07",
    }
}

pub fn event_label_08(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_08",
    }
}

pub fn event_label_09(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_09",
    }
}

pub fn event_label_10(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_10",
    }
}

pub fn event_label_11(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_11",
    }
}

pub fn event_label_12(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_12",
    }
}

pub fn event_label_13(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_13",
    }
}

pub fn event_label_14(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_14",
    }
}

pub fn event_label_15(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_15",
    }
}

pub fn event_label_16(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_16",
    }
}

pub fn event_label_17(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_17",
    }
}

pub fn event_label_18(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_18",
    }
}

pub fn event_label_19(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_19",
    }
}

pub fn event_label_20(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_20",
    }
}

pub fn event_label_21(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_21",
    }
}

pub fn event_label_22(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_22",
    }
}

pub fn event_label_23(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_23",
    }
}

pub fn event_label_24(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_24",
    }
}

pub fn event_label_25(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_25",
    }
}

pub fn event_label_26(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_26",
    }
}

pub fn event_label_27(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_27",
    }
}

pub fn event_label_28(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_28",
    }
}

pub fn event_label_29(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_29",
    }
}

pub fn event_label_30(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_30",
    }
}

pub fn event_label_31(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_31",
    }
}

pub fn event_label_32(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_32",
    }
}

pub fn event_label_33(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_33",
    }
}

pub fn event_label_34(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_34",
    }
}

pub fn event_label_35(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_35",
    }
}

pub fn event_label_36(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_36",
    }
}

pub fn event_label_37(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_37",
    }
}

pub fn event_label_38(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_38",
    }
}

pub fn event_label_39(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_39",
    }
}

pub fn event_label_40(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_40",
    }
}

pub fn event_label_41(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_41",
    }
}

pub fn event_label_42(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_42",
    }
}

pub fn event_label_43(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_43",
    }
}

pub fn event_label_44(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_44",
    }
}

pub fn event_label_45(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_45",
    }
}

pub fn event_label_46(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_46",
    }
}

pub fn event_label_47(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_47",
    }
}

pub fn event_label_48(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_48",
    }
}

pub fn event_label_49(kind: ParseEventKind) -> &'static str {
    match kind {
        ParseEventKind::DocumentOpened => "document_opened",
        ParseEventKind::HeaderParsed => "header_parsed",
        ParseEventKind::SectionLoaded => "section_loaded",
        ParseEventKind::ValidationStarted => "validation_started",
        ParseEventKind::RecoveryTriggered => "recovery_triggered",
        _ => "event_49",
    }
}
