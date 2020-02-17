#!/bin/bash

# Run with TARGET=<target> ./build_target.sh or TARGET=<target> ./build_target.sh --release

set -e

cargo build -p embedded-graphics --target $TARGET "$@"

pushd embedded-graphics
cargo build -p embedded-graphics --target $TARGET --all-features "$@"
popd

cargo build -p tinytga --target $TARGET "$@"
cargo build -p tinytga --target $TARGET --all-features "$@"
cargo build -p tinybmp --target $TARGET "$@"
cargo build -p tinybmp --target $TARGET --all-features "$@"
