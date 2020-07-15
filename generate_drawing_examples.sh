#!/bin/bash

set -e

ASSETS=./doc/assets

mkdir -p $ASSETS

cargo run --bin generate-drawing-examples | \
    rustfmt +nightly --config-path rustfmt.examples.toml | \
    sed -E -e 's@//! ?@@g' -e '/^# .*/d' -e '/pub mod dummy \{\}/d' - \
    > doc/drawing-examples.md
