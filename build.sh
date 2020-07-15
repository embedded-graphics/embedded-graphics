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

# Ensure that examples files are up to date
./generate_drawing_examples.sh
git diff --quiet doc/ || (
    echo "doc/ folder is not up to date"
    echo "Try running ./generate_drawing_examples.sh"
    echo "If any images have changed, run ./generate_examples_montage.sh to update the collage image too"
)

cargo doc --all-features
linkchecker --check-extern --ignore-url=^http target/doc/embedded_graphics/index.html
linkchecker --check-extern --ignore-url=^http target/doc/tinybmp/index.html
linkchecker --check-extern --ignore-url=^http target/doc/tinytga/index.html
linkchecker --check-extern --ignore-url=^http target/doc/embedded_graphics_simulator/index.html
