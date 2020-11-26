targets := "arm-unknown-linux-gnueabi armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu x86_64-unknown-linux-musl thumbv6m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf thumbv7m-none-eabi"

target_dir := "target"
# FIXME: `-cimg` suffix is temporary while moving to new CircleCI base images. Remove this suffix
# when upgrading the tag next time.
ci_build_image := "jamwaffles/circleci-embedded-graphics:1.40.0-cimg"

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
    cargo doc --workspace --features "{{all_features}}"

# Runs cargo-deadlinks on the docs
check-links: generate-docs
    cargo deadlinks --dir target/doc/embedded_graphics
    cargo deadlinks --dir target/doc/embedded_graphics_core

#----------------------
# README.md generation
# ---------------------

# Generate README.md for a single crate
generate-core-readme: (_build-readmes)
    cp {{target_dir}}/README-core.md embedded-graphics-core/README.md

# Generate README.md for a single crate
generate-readme: (_build-readmes)
    cp {{target_dir}}/README.md README.md

# Generate all READMEs
generate-readmes: generate-core-readme generate-readme

# Check READMEs
@check-readmes: (_build-readmes)
    diff -q {{target_dir}}/README.md README.md || ( \
        echo -e "\033[1;31mError:\033[0m README.md needs to be regenerated."; \
        echo -e "       Run 'just generate-readme' to regenerate.\n"; \
        exit 1 \
    )

    diff -q {{target_dir}}/README-core.md embedded-graphics-core/README.md || ( \
        echo -e "\033[1;31mError:\033[0m Core README.md needs to be regenerated."; \
        echo -e "       Run 'just generate-core-readme' to regenerate.\n"; \
        exit 1 \
    )

# Builds README.md for a single crate
_build-readmes:
    #!/usr/bin/env bash
    set -e -o pipefail
    echo "Building README.md"
    cargo readme | sed -E -f filter_readme.sed > {{target_dir}}/README.md

    echo "Building core README.md"
    cargo readme -r embedded-graphics-core | sed -E -f filter_readme.sed > {{target_dir}}/README-core.md

#--------
# Docker
#--------

# Generate the Docker image used by the CI pipeline
build-ci-image:
    docker build -t "{{ci_build_image}}" -f ./.circleci/Dockerfile --compress .

# Push the generated CI build environment image to Docker Hub
push-ci-image:
    docker push "{{ci_build_image}}"
