
#!/bin/bash -eu
cd "$SRC/nexform"
cargo fuzz build --offline -O
FUZZ_TARGET_DIR="fuzz/target/x86_64-unknown-linux-gnu/release"
for t in parser_fuzzer decoder_fuzzer state_machine_fuzzer; do
  cp "$FUZZ_TARGET_DIR/$t" "$OUT/"
done
