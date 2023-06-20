# Frontend Dockerfile
FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk cargo-watch
RUN cargo install --path .

EXPOSE 3000

CMD ["frontend"]