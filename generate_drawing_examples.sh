#!/bin/bash

set -e

ASSETS=./doc/assets

mkdir -p $ASSETS

cargo run --bin generate-drawing-examples | \
    rustfmt +nightly --config-path rustfmt.examples.toml | \
    sed -E -e 's@//! ?@@g' -e '/^# .*/d' -e '/pub mod dummy \{\}/d' - \
    > doc/drawing-examples.md

# Generate a collage of all screenshots
# `imagemagick` must be installed for this to work.
montage $ASSETS/draw*.png $ASSETS/display*.png -tile 6x2 -background none -geometry 128x128+4+4 miff:- | convert - -trim $ASSETS/all_drawing_ops.png