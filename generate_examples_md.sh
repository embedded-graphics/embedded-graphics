#!/bin/bash

set -e

ASSETS=./doc/assets

mkdir -p $ASSETS

cargo run --bin generate-example-screenshots | \
    rustfmt +nightly --config-path rustfmt.examples.toml | \
    sed -E -e 's@//! ?@@g' -e '/^# .*/d' -e '/pub mod dummy \{\}/d' - \
    > doc/primitives.md

# Generate a collage of all screenshots
# `imagemagick` must be installed for this to work.
montage $ASSETS/draw*.png $ASSETS/display*.png -tile 6x2 -background none $ASSETS/all_drawing_ops.png