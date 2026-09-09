# ONX Fuzzing

The ONX fuzz package provides coverage-guided regression targets for protocol
boundaries that process untrusted input. It implements TASK-018 using
`cargo-fuzz` and libFuzzer.

## Targets

- `boc_parser` parses arbitrary Bag-of-Cells encodings, including malformed
  cell entries and cyclic reference graphs.
- `tvm_execution` runs arbitrary bounded TVM bytecode against a fixed message
  and execution context, exercising decoder and stack-limit failure paths.
- `block_header` parses block headers and feeds arbitrary public-key/signature
  pairs through BFT vote verification.

Run a target locally after installing `cargo-fuzz`:

```sh
cargo install cargo-fuzz
cargo fuzz run boc_parser --manifest-path fuzz/Cargo.toml
```

To replay saved regressions, place corpus inputs under
`fuzz/corpus/<target>/` and run the corresponding target. Crash artifacts are
written below `fuzz/artifacts/`; inspect and minimize them before adding a
regression input to the corpus.
