//! Multi-stage NXF parse session orchestrating the full pipeline.


use crate::common::types::{
    has_flag, HeaderInfo, InterpretMode, ParseStats, SectionDescriptor, SectionFlags,
    SectionType, Span, Status,
};
use crate::core::arena::Arena;
use crate::core::object_pool::ObjectPool;
use crate::core::state_machine::{ProcessState, StateMachine};
use crate::decode::decoder::PayloadDecoder;
use crate::format::annotation_parser::AnnotationParser;
use crate::format::header::HeaderParser;
use crate::format::integrity_checker::IntegrityChecker;
use crate::format::metadata::MetadataDecoder;
use crate::format::section::SectionLoader;
use crate::format::section_catalog::SectionCatalog;
use crate::format::timeline::{TimelineBuilder, TimelineEventKind};
use crate::format::xref::XrefResolver;
use crate::runtime::event_bus::EventBus;
use crate::runtime::recovery::RecoveryManager;
use crate::validate::profile_validator::ProfileValidator;
use crate::validate::range_checker::RangeChecker;
use crate::validate::validator::DeferredValidator;

pub struct Session {
    arena: Arena,
    pool: ObjectPool,
    state_machine: StateMachine,
    metadata_decoder: MetadataDecoder,
    catalog: SectionCatalog,
    profile_validator: ProfileValidator,
    range_checker: RangeChecker,
    integrity_checker: IntegrityChecker,
    event_bus: EventBus,
    timeline_builder: TimelineBuilder,
    header: HeaderInfo,
    sections: Vec<SectionDescriptor>,
    document_owned: Vec<u8>,
    stats: ParseStats,
    pinned_section_id: Option<u16>,
    pipeline_active: bool,
}

impl Default for Session {
    fn default() -> Self { Self::new() }
}

impl Session {
    pub fn new() -> Self {
        let mut s = Self {
            arena: Arena::default(),
            pool: ObjectPool::default(),
            state_machine: StateMachine::default(),
            metadata_decoder: MetadataDecoder::default(),
            catalog: SectionCatalog,
            profile_validator: ProfileValidator::default(),
            range_checker: RangeChecker::default(),
            integrity_checker: IntegrityChecker::default(),
            event_bus: EventBus::default(),
            timeline_builder: TimelineBuilder::default(),
            header: HeaderInfo::default(),
            sections: Vec::new(),
            document_owned: Vec::new(),
            stats: ParseStats::default(),
            pinned_section_id: None,
            pipeline_active: false,
        };
        s.profile_validator.set_profile(ProfileValidator::detect_profile(&HeaderInfo::default()));
        s
    }

    pub fn process_input(&mut self, input: Span<'_>) -> Status {
        self.run_pipeline(input)
    }

    pub fn process_partial(&mut self, input: Span<'_>, finalize: bool) -> Status {
        self.store_document(input);
        let st = self.index_and_load(input);
        if st != Status::Ok { return st; }
        if finalize { self.drive_execution() } else { Status::Ok }
    }

    pub fn resume_after_recovery(&mut self) -> Status {
        if self.state_machine.current() != ProcessState::Recovering {
            return Status::StateError;
        }
        let section_id = self.state_machine.active_section();
        {
            let mut recovery = RecoveryManager::new(&mut self.arena, &mut self.pool, &mut self.state_machine);
            let _ = recovery.replay_from_section(section_id);
        }
        let _ = self.state_machine.transition(ProcessState::PayloadDecoding);
        self.timeline_builder.add_pipeline_event(
            TimelineEventKind::Recovery,
            Status::Ok,
            section_id,
            "resume_after_recovery",
        );
        if let Some(id) = self.pinned_section_id.or(Some(section_id)) {
            let data = self.pool.lookup(id).map(|s| s.data().data.to_vec()).unwrap_or_default();
if !data.is_empty() {
    let mode = self.state_machine.interpretation();
    if let Some(sec) = self.pool.lookup_mut(id) {
        let mut decoder = PayloadDecoder::new(&mut self.state_machine, &self.metadata_decoder);
        let _ = decoder.decode_section(sec, Span::new(&data));
        if mode == InterpretMode::Graph {
            unsafe {
                let _ = decoder.touch_cached_buffer(sec);
            }
        }
    }
    self.stats.decode_ops += 1;
}
        }
        self.run_executor()
    }

    pub fn stats(&self) -> &ParseStats { &self.stats }
    pub fn state(&self) -> ProcessState { self.state_machine.current() }
    pub fn header(&self) -> &HeaderInfo { &self.header }
    pub fn sections(&self) -> &[SectionDescriptor] { &self.sections }
    pub fn pool(&self) -> &ObjectPool { &self.pool }
    pub fn timeline(&self) -> &TimelineBuilder { &self.timeline_builder }

    pub fn handle_state_event(&mut self, code: u8) -> Status {
        self.state_machine.handle_event(code)
    }

    fn document_span(&self) -> Span<'_> {
        Span::new(&self.document_owned)
    }

    fn store_document(&mut self, input: Span<'_>) {
        self.document_owned = input.data.to_vec();
    }

    fn run_pipeline(&mut self, input: Span<'_>) -> Status {
        self.store_document(input);
        self.pipeline_active = true;
        let st = self.index_and_load(input);
        if st != Status::Ok { return st; }
        let st = self.run_structural_validation(input);
        if st != Status::Ok { return st; }
        let st = self.resolve_cross_refs();
        if st != Status::Ok { return st; }
        let st = self.drive_execution();
        self.pipeline_active = false;
        self.event_bus.flush();
        st
    }

    fn index_and_load(&mut self, input: Span<'_>) -> Status {
        let st = HeaderParser::parse(input, &mut self.header, &mut self.sections);
        if st != Status::Ok { return st; }
        self.timeline_builder.set_document_origin(self.header.creation_timestamp);
        self.stats.bytes_consumed = input.len();
        let _ = self.state_machine.handle_event(0x01);
        let _ = self.state_machine.handle_event(0x02);
        let mut loader = SectionLoader::new(&mut self.arena, &mut self.pool);
        let st = loader.load_all(input, &self.sections);
        if st != Status::Ok { return st; }
        self.stats.sections_parsed = self.sections.len();
        let sections = self.sections.clone();
        for desc in sections {
            self.timeline_builder.add_section_event(&desc, TimelineEventKind::SectionAdded);
            if desc.ty == SectionType::Metadata {
                if let Some(sec) = self.pool.lookup_mut(desc.section_id) {
                    let data = sec.data().data.to_vec();
                    self.metadata_decoder.decode(sec, Span::new(&data));
                    self.metadata_decoder.set_length_hint(desc.extended_length);
                    let mut fields = Vec::new();
                    let _ = self.metadata_decoder.decode_fields(Span::new(&data), &mut fields);
                    self.range_checker.add_from_metadata_fields(desc.section_id, &fields, 4);
                }
            }
            if desc.ty == SectionType::Annotation {
                if let Some(sec) = self.pool.lookup(desc.section_id) {
                    let mut ap = AnnotationParser::default();
                    if ap.parse(sec.data()) == Status::Ok {
                        for rec in ap.records() {
                            self.timeline_builder.add_annotation_event(rec);
                        }
                    }
                }
            }
            if desc.ty == SectionType::StateInit {
                if desc.length >= 1 && (desc.offset as usize) < input.len() {
                    let mode = input.data[desc.offset as usize];
                    self.state_machine.set_interpret_mode(match mode & 0x03 {
                        1 => InterpretMode::Structured,
                        2 => InterpretMode::Graph,
                        3 => InterpretMode::Stream,
                        _ => InterpretMode::Raw,
                    });
                }
            }
            if desc.ty == SectionType::DeferredValidate {
                if let Some(sec) = self.pool.lookup(desc.section_id) {
                    let mut validator = DeferredValidator::new(&self.state_machine);
                    let _ = validator.validate_section(sec, sec.data());
                }
            }
        }
        let _ = self.state_machine.handle_event(0x03);
        Status::Ok
    }

    fn run_structural_validation(&mut self, input: Span<'_>) -> Status {
        self.range_checker.clear();
        self.range_checker.set_document_size(input.len());
        self.range_checker.set_strict(false);
        self.range_checker.add_from_descriptors(&self.sections);
        let _ = self.range_checker.validate_all();
        self.integrity_checker.clear();
        self.integrity_checker.add_from_descriptors(&self.sections);
        self.integrity_checker.set_document(input);
        let _ = self.integrity_checker.verify();
        let st = self.catalog.validate_document_layout(&self.sections);
        if st != Status::Ok { return st; }
        self.profile_validator.set_profile(ProfileValidator::detect_profile(&self.header));
        self.profile_validator.validate_header(&self.header)
    }

    fn resolve_cross_refs(&mut self) -> Status {
        let sections = self.sections.clone();
        for desc in &sections {
            if desc.ty == SectionType::Xref {
                if let Some(sec) = self.pool.lookup(desc.section_id) {
                    let data = sec.data().data.to_vec();
                    let st = XrefResolver.resolve(Span::new(&data), &mut self.pool);
                    if st != Status::Ok && !has_flag(desc.flags, SectionFlags::Optional) {
                        return st;
                    }
                    self.stats.xrefs_resolved += 1;
                }
            }
        }
        let _ = self.state_machine.handle_event(0x04);
        for desc in sections {
            if desc.ty == SectionType::Payload || desc.ty == SectionType::CompressedBlob {
                self.state_machine.set_active_section(desc.section_id);
                if self.pool.lookup(desc.section_id).is_some() {
                    if has_flag(desc.flags, SectionFlags::Recoverable) {
                        self.pinned_section_id = Some(desc.section_id);
                        if let Some(idx) = self.pool.slots.iter().position(|s| s.descriptor().section_id == desc.section_id) {
                            self.state_machine.cache_object_index(idx, 0);
                        }
                    }
                }
            }
        }
        Status::Ok
    }

    fn drive_execution(&mut self) -> Status {
        let st = self.run_executor();
        if st == Status::ChecksumMismatch {
            self.stats.recovery_count += 1;
            let sid = self.state_machine.active_section();
            let doc = self.document_owned.clone();
            {
                let mut recovery = RecoveryManager::new(&mut self.arena, &mut self.pool, &mut self.state_machine);
                let _ = recovery.handle_checksum_failure(sid, Span::new(&doc));
            }
            return self.resume_after_recovery();
        }
        st
    }

    fn run_executor(&mut self) -> Status {
        {
            let mut decoder = PayloadDecoder::new(&mut self.state_machine, &self.metadata_decoder);
            let limit = self.pool.active_count() + 16;
            for i in 0..limit {
                let should = self.pool.get_by_index(i).map(|s| {
                    s.is_active() && matches!(s.descriptor().ty, SectionType::Payload | SectionType::CompressedBlob | SectionType::ObjectGraph)
                }).unwrap_or(false);
                if !should { continue; }
                if let Some(section) = self.pool.get_by_index_mut(i) {
                    let data = section.data().data.to_vec();
                    let st = decoder.decode_section(section, Span::new(&data));
                    if st != Status::Ok { return st; }
                }
            }
        }
        let _ = self.state_machine.transition(ProcessState::Validating);
        {
            let validator = DeferredValidator::new(&self.state_machine);
            for i in 0..self.pool.active_count() + 16 {
                if let Some(section) = self.pool.get_by_index(i) {
                    if section.is_active() {
                        let data = section.data();
                        let st = validator.validate_section(section, data);
                        if st != Status::Ok { return st; }
                    }
                }
            }
        }
        let _ = self.state_machine.transition(ProcessState::Executing);
        Status::Ok
    }
}
