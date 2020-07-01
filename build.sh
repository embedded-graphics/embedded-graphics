#!/bin/bash

set -xe

CRATES=("embedded-graphics" "simulator" "tinybmp" "tinytga")

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

# Generate and check readmes. If generated readme has changed from what's committed, this will fail.
for crate in "${CRATES[@]}"; do
    ./readme.sh --check "$crate"
done

cargo doc --all-features
linkchecker --check-extern --ignore-url=^http \
    target/doc/embedded_graphics/index.html \
    target/doc/tinybmp/index.html \
    target/doc/tinytga/index.html \
    target/doc/embedded_graphics_simulator/index.html
