targets := "arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu x86_64-unknown-linux-musl thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi"

target_dir := "target"
ci_build_image := "jamwaffles/circleci-embedded-graphics:1.40.0-2"

# list of all features except criterion
all_features := "nalgebra_support fixed"

#----------
# Building
#----------

build: check-formatting test test-all build-benches check-readmes check-links

# Build the benches
build-benches:
    cargo bench --workspace --features "criterion" --no-run

# Run the benches
bench *args:
    cargo bench --workspace --features "criterion" {{args}}

# Run cargo test in release mode
test:
    cargo test --workspace --release

# Run cargo test in release mode with all features enabled
test-all:
    cargo test --workspace --release --features "{{all_features}}"

# Check the formatting
check-formatting:
    cargo fmt --all -- --check

# Cross compiles embedded-graphics for a target
build-target target *args:
    cargo build --workspace --target {{target}} {{args}}
    cargo build --workspace --target {{target}} --features "{{all_features}}" {{args}}

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
    cargo doc --workspace --features "{{all_features}}" --no-deps

# Runs cargo-deadlinks on the docs
check-links: generate-docs
    cargo deadlinks --dir target/doc/embedded_graphics
    cargo deadlinks --dir target/doc/embedded_graphics_core

#----------------------
# README.md generation
#----------------------

# Generate README.md for a single crate
generate-readme crate: (_build-readme crate)
    cp {{target_dir}}/{{crate}}_README.md {{crate}}/README.md

# Check README.md for a single crate
@check-readme crate: (_build-readme crate)
    diff -q {{target_dir}}/{{crate}}_README.md ./{{crate}}/README.md || ( \
        echo -e "\033[1;31mError:\033[0m README.md for {{crate}} needs to be regenerated."; \
        echo -e "       Run 'just generate-readmes' to regenerate.\n"; \
        exit 1 \
    )

# Generate README.md for all crates
generate-readmes: (generate-readme ".") (generate-readme "./core")

# Checks README.md for all crates
check-readmes: (check-readme ".") (check-readme "./core")

# Builds README.md for a single crate
_build-readme crate:
    #!/usr/bin/env bash
    set -e -o pipefail
    mkdir -p {{target_dir}}/readme
    echo "Building README.md for {{crate}}"
    cargo readme -r {{crate}} | sed -E -f filter_readme.sed > {{target_dir}}/{{crate}}_README.md

#----------------
# Font conversion
#----------------

convert-fonts:
    cd tools/convert-fonts && cargo run --release
    find src/mono_font/ -name generated.rs -exec rustfmt {} \;

#--------
# Docker
#--------

# Generate the Docker image used by the CI pipeline
build-ci-image:
    docker build -t "{{ci_build_image}}" -f ./.circleci/Dockerfile --compress .

# Push the generated CI build environment image to Docker Hub
push-ci-image:
    docker push "{{ci_build_image}}"

# -------
# Release
# -------
# Release embedded-graphics-core
release-core *args:
    cargo release --workspace --exclude embedded-graphics --dependent-version fix {{args}}

# Release embedded-graphics
release-e-g *args:
    cargo release --package embedded-graphics --dependent-version fix {{args}}
