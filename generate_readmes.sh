#!/bin/bash

set -xe

CRATES=("embedded-graphics" "simulator" "tinybmp" "tinytga")


for crate in "${CRATES[@]}"; do
    ./readme.sh "$crate" > "$crate/README.md"
done
