targets := "arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu x86_64-unknown-linux-musl thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi"

target_dir := "target"

doc_dir := "doc"
doc_assets_dir := doc_dir + "/assets"

#----------
# Building
#----------

build: check-formatting check-drawing-examples build-without-fmt-check

build-without-fmt-check: test test-all check-readmes check-links build-benches

# Build the benches
build-benches:
    cargo bench --workspace --no-run

# Run the benches
bench *args:
    cargo bench --workspace {{args}}

# Run cargo test
test:
    cargo test --workspace

# Run cargo test with all features enabled
test-all:
    cargo test --workspace --all-features

# Check the formatting
check-formatting:
    cargo fmt --all -- --check

# Cross compiles embedded-graphics for a target
build-target target *args:
    cargo build --workspace --target {{target}} {{args}}
    cargo build --workspace --target {{target}} --all-features {{args}}

# Cross compiles embedded-graphics for all targets
build-targets *args:
    #!/usr/bin/env bash
    set -e

    for target in {{targets}}; do just build-target $target {{args}}; done

# Install all targets used in the `build-targets` command
install-targets:
    #!/usr/bin/env bash
    set -e

    sysroot=$(rustc --print sysroot)

    for target in {{targets}}; do
      if [[ ! "$sysroot" =~ "$target" ]]; then
        rustup target add $target
      else
        echo "Target $target is already installed"
      fi
    done

#------
# Docs
#------

# Generates the docs
generate-docs:
    cargo clean --doc
    cargo doc --workspace --all-features --no-deps

# Checks for broken links in cargo docs and readmes
check-links: generate-docs generate-readmes
    cargo deadlinks --ignore-fragments --dir target/doc/embedded_graphics
    cargo deadlinks --ignore-fragments --dir target/doc/embedded_graphics_core

    cd tools/check-md-refs && cargo run -- '../../core/README.md'
    cd tools/check-md-refs && cargo run -- '../../README.md'

    lychee --exclude=circleci.com --verbose --exclude='LICENSE*' -- './**/README.md'

# Generate drawing examples in the doc directory
generate-drawing-examples:
    cd tools/generate-drawing-examples && cargo run
    rustfmt src/examples.rs

# Checks if drawing examples are up to date
check-drawing-examples: generate-drawing-examples
    git diff --quiet doc/ || ( \
        echo "src/examples.rs is not up to date" \
        echo "Try running 'just generate-drawing-examples'." \
        echo "If any images have changed, run 'just generate-drawing-examples-montage' to update the collage image too" \
    )

# Generate a collage of all drawing example screenshots
generate-drawing-examples-montage:
    # `imagemagick` must be installed for this to work.
    montage \
        {{doc_assets_dir}}/draw*.png \
        -tile 6x2 -background none -geometry 128x128+4+4 miff:- | \
    convert - -trim {{doc_assets_dir}}/all_drawing_ops.png

#----------------------
# README.md generation
#----------------------

# Generate README.md for a single crate
generate-readme crate: (_build-readme crate)
    #!/usr/bin/env bash
    set -euo pipefail
    CRATE_DIR=$(dirname $(find . -name Cargo.toml -exec grep -l 'name = "{{crate}}"' {} \;))
    cp "{{target_dir}}/README-{{crate}}.md" "$CRATE_DIR/README.md"

# Check README.md for a single crate
@check-readme crate: (_build-readme crate)
    #!/usr/bin/env bash
    set -euo pipefail
    CRATE_DIR=$(dirname $(find . -name Cargo.toml -exec grep -l 'name = "{{crate}}"' {} \;))
    diff -q "{{target_dir}}/README-{{crate}}.md" "$CRATE_DIR/README.md" || ( \
        echo -e "\033[1;31mError:\033[0m README.md for {{crate}} needs to be regenerated."; \
        echo -e "       Run 'just generate-readmes' to regenerate.\n"; \
        exit 1 \
    )

# Generate README.md for all crates
generate-readmes: (generate-readme "embedded-graphics") (generate-readme "embedded-graphics-core")

# Checks README.md for all crates
check-readmes: (check-readme "embedded-graphics") (check-readme "embedded-graphics-core")

# Builds README.md for a single crate
_build-readme crate:
    #!/usr/bin/env bash
    set -e -o pipefail
    mkdir -p {{target_dir}}
    echo "Building README.md for {{crate}}"
    CRATE_DIR=$(dirname $(find . -name Cargo.toml -exec grep -l 'name = "{{crate}}"' {} \;))
    cargo readme -r "$CRATE_DIR" | sed -E -f "filter-readme.sed" > "{{target_dir}}/README-{{crate}}.md"

#----------------
# Font conversion
#----------------

convert-fonts:
    cd tools/convert-fonts && cargo run --release
    find src/mono_font/ -name generated.rs -exec rustfmt {} \;

#--------
# Release
#--------

# Release embedded-graphics-core
release-core +args:
    cargo release --workspace --exclude embedded-graphics --dependent-version fix {{args}}

# Release embedded-graphics
release-e-g +args:
    cargo release --package embedded-graphics --dependent-version fix {{args}}
