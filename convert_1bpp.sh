#!/bin/sh

set -e

# Convert a PNG to a 1BPP greyscale image
convert $1.png -depth 1 gray:"$1_1bpp.raw"