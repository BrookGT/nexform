# NXF Format Specification (v2.4)

NXF is a binary container format for bundling structured application data into a
single file. It is used by **nexform** for offline content packs, sync bundles, and
cached API snapshots.

## Design goals

1. **Self-contained** — all sections, index, and checksums in one file
2. **Streamable** — header and section table allow incremental loading
3. **Typed** — each section has an explicit type and schema validation path
4. **Linkable** — xref sections bind metadata fields to payload regions
5. **Recoverable** — optional recoverable flag enables partial replay after errors

## File layout

```
+-------------------+
| Header (32 bytes) |
+-------------------+
| Section table     |  (inline or via index offset)
+-------------------+
| Section payloads  |
+-------------------+
```

### Header

| Offset | Size | Field |
|--------|------|-------|
| 0 | 4 | Magic `NXF1` (0x3158464E) |
| 4 | 2 | Version major |
| 6 | 2 | Version minor |
| 8 | 4 | Document flags |
| 12 | 2 | Section count |
| 14 | 4 | Index offset (0 if inline table) |
| 18 | 4 | Creation timestamp (Unix) |
| 22 | 10 | Reserved |

### Document flags

| Bit | Name | Meaning |
|-----|------|---------|
| 0 | Streaming | Allow incremental decode |
| 1 | Indexed | Section table at index offset |
| 2 | Recoverable | Enable recovery replay |
| 3 | Polymorphic | Polymorphic payload interpretation |

### Section descriptor (base 20 bytes)

| Offset | Size | Field |
|--------|------|-------|
| 0 | 1 | Section type |
| 1 | 1 | Flags |
| 2 | 2 | Section ID |
| 4 | 4 | Payload offset |
| 8 | 4 | Payload length |
| 12 | 4 | CRC32 checksum |
| 16 | 4 | Extended length (if MergeLengths flag) |
| 20 | 2 | Xref target (if CrossLinked flag) |

### Section types

| Value | Name | Purpose |
|-------|------|---------|
| 0x01 | header_info | Document-level info |
| 0x02 | metadata | Key/value field table |
| 0x03 | payload | Typed object body |
| 0x04 | xref | Cross-section bindings |
| 0x05 | index | Secondary index |
| 0x06 | annotation | Tags, labels, timestamps |
| 0x07 | compressed_blob | Compressed payload |
| 0x08 | state_init | Parser interpret mode |
| 0x09 | transform_chain | Post-decode transforms |
| 0x0A | deferred_validate | Checksum validated late |
| 0x0B | object_graph | Graph-structured objects |

### Section flags

| Bit | Name |
|-----|------|
| 0 | Optional — parse may skip on error |
| 1 | Compressed |
| 2 | Encrypted (XOR layer) |
| 3 | Recoverable |
| 4 | MergeLengths — extended length follows descriptor |
| 5 | DeferredChecksum |
| 6 | CrossLinked |
| 7 | Polymorphic |

## Metadata section

```
uint32  field_count
repeat field_count times:
  uint16  field_id
  uint16  type_code
  uint32  value_offset
  uint32  value_length
uint32  extended_capacity   (optional)
```

## Xref entry (12 bytes)

```
uint16  source_section_id
uint16  target_section_id
uint32  field_offset
uint8   binding_flags
uint8   reserved
uint16  resolved_length
```

Binding flags: length override (0x01), type redirect (0x02), deferred (0x04).

## Processing pipeline

1. Parse header and section table
2. Load section payloads into pool
3. Structural validation (ranges, integrity, profile)
4. Resolve xref bindings
5. Decode payloads (decompress, transforms)
6. Validate checksums (immediate or deferred)
7. Recovery replay if recoverable sections fail validation

## Version history

- **2.4** — deferred validation, object graph sections, hash chains
- **2.3** — streaming API, annotation timestamps
- **2.0** — initial indexed layout
