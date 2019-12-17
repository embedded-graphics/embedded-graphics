#!/bin/bash

set -e

cargo clean --doc

cargo fmt --all -- --check
cargo test --release
cargo test --release --all-features
cargo bench --no-run

cargo doc --all-features
linkchecker target/doc/embedded_graphics/index.html
linkchecker target/doc/tinybmp/index.html
linkchecker target/doc/tinytga/index.html
linkchecker target/doc/embedded_graphics_simulator/index.html
