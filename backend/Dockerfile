# Backend Dockerfile
FROM rust:latest

WORKDIR /usr/src/app

COPY . .
RUN cargo install sqlx-cli --features postgres
RUN cargo install --path .

EXPOSE 8000

CMD ["backend"]
