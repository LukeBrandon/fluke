FROM rustlang/rust:nightly

WORKDIR /usr/src/app

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk cargo-watch
COPY . .
RUN cargo build 

EXPOSE 3000

CMD ["frontend"]