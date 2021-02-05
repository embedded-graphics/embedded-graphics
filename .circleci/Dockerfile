FROM cimg/rust:1.40.0 AS tools-builder

# Update rust using rustup.
#
# We cannot use a newer cimg/rust version to build the tools,
# because newer images use an incompatible version of libc.
RUN rustup default 1.48.0

RUN cargo install --root /home/circleci/project just --version ^0.8
RUN cargo install --root /home/circleci/project cargo-readme --version ^3.2
RUN cargo install --root /home/circleci/project cargo-deadlinks --version ^0.8

FROM cimg/rust:1.40.0

USER root

RUN apt-get update && apt-get install -y \
    libsdl2-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

USER circleci

RUN rustup toolchain add nightly --profile minimal --component rustfmt

COPY --from=tools-builder /home/circleci/project/* /home/circleci/.cargo/bin/
