# Use the official Rust image as a base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/orderbook /usr/local/bin/orderbook

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/orderbook"]