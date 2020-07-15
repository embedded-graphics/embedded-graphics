#!/bin/bash

set -e

ASSETS=./doc/assets

# Generate a collage of all screenshots
# `imagemagick` must be installed for this to work.
montage $ASSETS/draw*.png $ASSETS/display*.png -tile 6x2 -background none -geometry 128x128+4+4 miff:- | convert - -trim $ASSETS/all_drawing_ops.png
