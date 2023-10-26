# the env used to develop on Windows
FROM rust
RUN rustup default stable
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
WORKDIR /workspace
ENTRYPOINT ["/bin/bash"]
