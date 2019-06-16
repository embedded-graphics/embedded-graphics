#!/bin/bash

# Run with TARGET=<target> ./build_target.sh or TARGET=<target> ./build_target.sh --release

set -e

cargo build -p embedded-graphics --target $TARGET "$@"
cargo build -p embedded-graphics --target $TARGET --features 'bmp tga nalgebra' "$@"

cargo build -p tinytga --target $TARGET "$@"
cargo build -p tinybmp --target $TARGET "$@"
