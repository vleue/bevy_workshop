FROM rust:1.82-bullseye

RUN rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm wasm-bindgen-cli cargo-watch basic-http-server

RUN git clone https://github.com/vleue/bevy_workshop
RUN cd bevy_workshop && cargo build --release --target wasm32-unknown-unknown
