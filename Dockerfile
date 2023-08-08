FROM rust:latest as builder
WORKDIR /usr/src/backend

# Copy the Cargo files and create a dummy main.rs to build dependencies
COPY backend/Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy main.rs and copy the source code
RUN rm src/main.rs
COPY backend/src ./src
RUN cargo build --release

FROM debian:buster-slim

# Copy the compiled binary from the builder stage to this new stage
COPY --from=builder /usr/src/backend/target/release/backend-axum /usr/local/bin/

# Command to run the application
CMD ["backend-axum"]
