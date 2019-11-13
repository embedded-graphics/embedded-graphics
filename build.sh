#!/bin/bash

set -e

cargo fmt --all -- --check
cargo test --release

pushd embedded-graphics
cargo test --release --all-features
popd

cargo doc --all-features
cargo bench --no-run

linkchecker target/doc/embedded_graphics/index.html
linkchecker target/doc/tinybmp/index.html
linkchecker target/doc/tinytga/index.html
linkchecker target/doc/embedded_graphics_simulator/index.html
