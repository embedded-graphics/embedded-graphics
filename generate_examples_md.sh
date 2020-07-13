#!/bin/bash

cargo run --bin generate-example-screenshots | rustfmt +nightly --config-path rustfmt.examples.toml | sed -E -e 's@//! ?@@g' -e '/^# .*/d' -e '/pub mod dummy \{\}/d' - > EXAMPLES.md
