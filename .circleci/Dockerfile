FROM cimg/rust:1.40.0

RUN sudo apt update && sudo apt install -y libsdl2-dev libssl-dev && sudo apt clean

RUN cargo install just --version ^0.6
RUN cargo install cargo-readme --version ^3.2

# MSRV: Upgrade to 0.6 when >= 1.45.0
RUN cargo install cargo-deadlinks --version ^0.4

RUN rustup toolchain add nightly -c rustfmt

