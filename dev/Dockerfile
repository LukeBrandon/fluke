FROM rust:latest
RUN echo "export PATH=$PATH:/usr/local/cargo/bin" >> /root/.bashrc
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk cargo-watch