FROM rustlang/rust:nightly

WORKDIR /usr/src/app

RUN cargo install sqlx-cli --features postgres
COPY . .
RUN cargo build 

EXPOSE 8000

CMD ["backend"]
