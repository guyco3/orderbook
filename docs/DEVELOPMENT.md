# Local Development Guide

### Building from Source
We use `maturin` to bridge our Rust engine with Python.

1. **Incremental Development:**
   Use `maturin develop` to sync Rust changes to your local python environment.
   
2. **Performance Testing:**
   Always build with the `--release` flag to enable Rust optimizations:
   `maturin develop --release`

3. **CI Compliance:**
   Before pushing, ensure the code is formatted and linted:
   `cargo fmt && cargo clippy -- -D warnings`