//! Export parsed NXF documents to alternate formats.


use crate::api::session::Session;
use crate::common::buffer::GrowableBuffer;
use crate::common::types::Status;
use crate::format::document_writer::DocumentWriter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Binary,
    SectionDump,
    MetadataJson,
    TimelineText,
}

pub struct ExportOptions {
    pub format: ExportFormat,
    pub include_annotations: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self { format: ExportFormat::Binary, include_annotations: true }
    }
}

pub fn export_document(session: &Session, opts: ExportOptions, out: &mut GrowableBuffer) -> Status {
    match opts.format {
        ExportFormat::Binary => {
            let mut writer = DocumentWriter::default();
            if writer.begin(session.header(), session.sections()) != Status::Ok {
                return Status::InternalError;
            }
            if out.append(writer.finish()) != Status::Ok {
                return Status::AllocationFailed;
            }
            Status::Ok
        }
        ExportFormat::SectionDump => export_section_dump(session, out),
        ExportFormat::MetadataJson => export_metadata_json(session, out),
        ExportFormat::TimelineText => export_timeline_text(session, out),
    }
}

fn export_section_dump(session: &Session, out: &mut GrowableBuffer) -> Status {
    for desc in session.sections() {
        let line = alloc::format!(
            "section {} type {} offset {} len {}\n",
            desc.section_id,
            desc.ty as u8,
            desc.offset,
            desc.length
        );
        if out.append(line.as_bytes()) != Status::Ok {
            return Status::AllocationFailed;
        }
    }
    Status::Ok
}

fn export_metadata_json(session: &Session, out: &mut GrowableBuffer) -> Status {
    let json = alloc::format!(
        "{{\"magic\":{},\"sections\":{}}}\n",
        session.header().magic,
        session.sections().len()
    );
    if out.append(json.as_bytes()) != Status::Ok {
        return Status::AllocationFailed;
    }
    Status::Ok
}

fn export_timeline_text(session: &Session, out: &mut GrowableBuffer) -> Status {
    for ev in session.timeline().events() {
        let line = alloc::format!("{} sid={} {}\n", ev.timestamp, ev.section_id, ev.detail);
        if out.append(line.as_bytes()) != Status::Ok {
            return Status::AllocationFailed;
        }
    }
    Status::Ok
}
