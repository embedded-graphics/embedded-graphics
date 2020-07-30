FROM circleci/rust:1.40.0

RUN sudo apt update && sudo apt install -y libsdl2-dev && sudo apt clean

RUN cargo install cargo-readme just cargo-deadlinks

RUN rustup toolchain add nightly -c rustfmt

