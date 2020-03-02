#!/bin/bash

OUTPUT_DIR=screenshots

mkdir -p $OUTPUT_DIR

for EXAMPLE in examples/*.rs
do
    NAME=$(basename "$EXAMPLE" .rs)
    PNG_FILE="$OUTPUT_DIR/$NAME.png"

    echo "Generating '$NAME' screenshot"

    cargo run --features dump-png --example "$NAME"
    mv dump.png "$PNG_FILE"
done
