crates := "embedded-graphics simulator tinybmp tinytga"
targets := "arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu x86_64-unknown-linux-musl thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi"

target_dir := "target"
doc_dir := "doc"
doc_assets_dir := doc_dir + "/assets"
screenshots_dir := target_dir + "/screenshots"
ci_build_image := "jamwaffles/circleci-embedded-graphics:1.40.0"

#----------
# Building
#----------

build: check-formatting test test-all build-benches build-simulator check-drawing-examples check-readmes check-links

# Build the benches
build-benches:
    cargo bench --no-run

# Build the simulator
build-simulator:
    cd simulator; \
    cargo build --release --no-default-features

# Run cargo test in release mode
test:
    cargo test --release

# Run cargo test in release mode with all features enabled
test-all:
    cargo test --release --all-features

# Check the formatting
check-formatting:
    cargo fmt --all -- --check

# Cross compiles embedded-graphics, tinytga and tinybmp for a target
build-target target *args:
    cargo build -p embedded-graphics --target {{target}} {{args}}
    cd embedded-graphics; cargo build -p embedded-graphics --target {{target}} --all-features {{args}}

    cargo build -p tinytga --target {{target}} {{args}}
    cargo build -p tinytga --target {{target}} --all-features {{args}}

    cargo build -p tinybmp --target {{target}} {{args}}
    cargo build -p tinybmp --target {{target}} --all-features {{args}}

# Cross compiles embedded-graphics, tinytga and tinybmp for all targets
build-targets *args:
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
    cargo doc --all-features

# Runs cargo-deadlinks on the docs
check-links: generate-docs
    cargo deadlinks

#----------------------
# README.md generation
# ---------------------

# Generate README.md for a single crate
generate-readme crate: (_build-readme crate)
    cp {{target_dir}}/{{crate}}_README.md {{crate}}/README.md

# Check README.md for a single crate
@check-readme crate: (_build-readme crate)
    diff -q {{target_dir}}/{{crate}}_README.md ./{{crate}}/README.md || ( \
        echo -e "\033[1;31mError:\033[0m README.md for {{crate}} needs to be regenerated."; \
        echo -e "       Run 'just generate-readme {{crate}}' to regenerate.\n"; \
        exit 1 \
    )

# Generate README.md for all crates
generate-readmes:
    for crate in {{crates}}; do just generate-readme $crate; done

# Checks README.md for all crates
check-readmes:
    for crate in {{crates}}; do just check-readme $crate; done

# Builds README.md for a single crate
_build-readme crate:
    #!/usr/bin/env bash
    set -e -o pipefail
    mkdir -p {{target_dir}}/readme
    echo "Building README.md for {{crate}}"
    cargo readme -r {{crate}} | sed -E -f filter_readme.sed > {{target_dir}}/{{crate}}_README.md

#----------
# Examples
#----------

# Generates the drawing examples screenshots and markdown file
generate-drawing-examples:
    #!/usr/bin/env bash
    set -ex -o pipefail
    mkdir -p {{doc_assets_dir}}
    cargo run --bin generate-drawing-examples | \
        rustfmt +nightly --config-path rustfmt.examples.toml | \
        sed -E -e 's@//! ?@@g' -e '/^# .*/d' -e '/pub mod dummy \{\}/d' - \
        > {{doc_dir}}/drawing-examples.md

# Checks if drawing examples are up to date
check-drawing-examples: generate-drawing-examples
    git diff --quiet doc/ || ( \
        echo "doc/ folder is not up to date" \
        echo "Try running 'just generate-drawing-examles'." \
        echo "If any images have changed, run just generate-drawing-examples-montage' to update the collage image too" \
    )

# Generate a collage of all drawing example screenshots
generate-drawing-examples-montage:
    # `imagemagick` must be installed for this to work.
    montage \
        {{doc_assets_dir}}/draw*.png \
        {{doc_assets_dir}}/display*.png \
        -tile 6x2 -background none -geometry 128x128+4+4 miff:- | \
    convert - -trim {{doc_assets_dir}}/all_drawing_ops.png

# Generates a screenshot of an example
generate-example-screenshot example:
    @mkdir -p "{{screenshots_dir}}"
    # Generating {{example}} screenshot
    EG_SIMULATOR_DUMP="{{screenshots_dir}}/{{example}}.png" \
    cargo run --example {{example}}

# Generates screenshots of all examples
@generate-example-screenshots:
    for example in simulator/examples/*.rs; do \
        just generate-example-screenshot $(basename "$example" .rs); \
    done

#--------
# Docker
#--------

# Generate the Docker image used by the CI pipeline
build-ci-image:
    docker build -t "{{ci_build_image}}" -f ./.circleci/Dockerfile --compress .

# Push the generated CI build environment image to Docker Hub
push-ci-image:
    docker push "{{ci_build_image}}"
