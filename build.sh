#!/bin/bash

set -xe

cargo clean --doc

cargo fmt --all -- --check
cargo test --release
cargo test --release --all-features
cargo bench --no-run

pushd simulator
cargo build --release --no-default-features
popd

cargo doc --all-features
linkchecker --check-extern --ignore-url=^http target/doc/embedded_graphics/index.html
linkchecker --check-extern --ignore-url=^http target/doc/tinybmp/index.html
linkchecker --check-extern --ignore-url=^http target/doc/tinytga/index.html
linkchecker --check-extern --ignore-url=^http target/doc/embedded_graphics_simulator/index.html
