//! Cross-reference resolver binding metadata to payload regions.


use crate::common::buffer::ByteReader;
use crate::common::types::{Span, Status, XrefEntry};
use crate::core::object_pool::ObjectPool;

pub struct XrefResolver;

impl XrefResolver {
    pub fn resolve(&self, data: Span<'_>, pool: &mut ObjectPool) -> Status {
        let mut reader = ByteReader::new(data);
        let count = match reader.read_u32_le() {
            Ok(v) => v,
            Err(e) => return e,
        };
        for _ in 0..count {
            let source_id = match reader.read_u16_le() { Ok(v) => v, Err(e) => return e };
            let target_id = match reader.read_u16_le() { Ok(v) => v, Err(e) => return e };
            let field_offset = match reader.read_u32_le() { Ok(v) => v, Err(e) => return e };
            let binding_flags = match reader.read_u8() { Ok(v) => v, Err(e) => return e };
            let _ = match reader.read_u8() { Ok(v) => v, Err(e) => return e };
            let resolved_length = match reader.read_u16_le() { Ok(v) => v, Err(e) => return e };
            let _entry = XrefEntry { source_id, target_id, field_offset, resolved_length: u32::from(resolved_length), binding_flags };
            if let Some(target) = pool.lookup_mut(target_id) {
                target.descriptor_mut().extended_length = u32::from(resolved_length);
            }
        }
        Status::Ok
    }
}
