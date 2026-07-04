# nexform

**nexform** is a Rust library for reading, writing, and validating **NXF** (Nexus
Exchange Format) document bundles. NXF packages structured application data —
metadata records, typed payloads, cross-references, annotations, and optional
compressed blobs — into a single binary file suitable for offline sync, local
caching, and incremental streaming.

Typical uses:

- Mobile/desktop apps shipping offline content packs
- Service workers caching structured API responses
- Edge nodes exchanging configuration and telemetry bundles
- Build pipelines archiving versioned artifact sets with integrity checks

## Features

- Multi-stage parse pipeline with deferred checksum validation
- Section-level cross-references and length binding
- Streaming ingestion for large bundles
- Integrity verification (CRC32 regions, hash chains)
- Range and profile validation
- Export to binary, section dump, metadata JSON, or timeline text
- Optional recoverable sections for partial replay after validation failure

## Building

### Library + fuzz harnesses (ClusterFuzzLite)

```bash
export SRC=$PWD
export WORK=/tmp/nxf-build
export OUT=/tmp/nxf-out
export CXX=clang++
export CXXFLAGS="-fsanitize=address -g"
export LIB_FUZZING_ENGINE="-fsanitize=fuzzer,address"
mkdir -p $WORK $OUT
bash .clusterfuzzlite/build.sh
```

### Inspector CLI

```bash
cargo build --workspace
./bin/nexform-cli validate sample.nxf
./bin/nexform-cli dump sample.nxf
./bin/nexform-cli export-json sample.nxf
```

## API overview

```cpp
#include "src/api/parser.h"

nex::Parser parser;
parser.ParseDocument(data, size);           // full pipeline

nex::ExportOptions opts;
opts.format = nex::ExportFormat::kSectionDump;
nex::GrowableBuffer out;
nex::ExportDocument(&parser.GetSession(), opts, &out);
```

See [docs/FORMAT.md](docs/FORMAT.md) for the on-disk layout.

## Project layout

```
src/
  api/        Parser, session, streaming, export
  common/     Buffers, checksums, diagnostics
  core/       Arena, object pool, state machine, cache
  decode/     Payload decode, compression, transforms
  format/     Header, sections, metadata, xref, writer
  runtime/    Executor, recovery, jobs, events
  validate/   Schema, constraints, profiles, ranges
tools/        nexform-cli CLI
fuzz/         libFuzzer entry points
```

## License

MIT — see [LICENSE](LICENSE).
