#!/bin/bash

set -e

cargo fmt --all -- --check
cargo test --release
cargo test --release --all-features
cargo bench --no-run

cargo doc --all-features
linkchecker target/doc/embedded_graphics
linkchecker target/doc/tinybmp
linkchecker target/doc/tinytga
