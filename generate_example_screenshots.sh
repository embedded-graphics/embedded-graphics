#!/bin/bash

OUTPUT_DIR=target/screenshots

mkdir -p $OUTPUT_DIR

for EXAMPLE in simulator/examples/*.rs
do
    NAME=$(basename "$EXAMPLE" .rs)

    echo "Generating '$NAME' screenshot"

    export EG_SIMULATOR_DUMP="$OUTPUT_DIR/$NAME.png"
    cargo run --example "$NAME"
done
