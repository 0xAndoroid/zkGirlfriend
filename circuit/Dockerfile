# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new directory for our app
WORKDIR /app

# Copy the `Cargo.toml` and `Cargo.lock` files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the `lib` directory from the context into the container
COPY lib /app/lib/

# Copy the rest of the application code
COPY backend/src /app/src

# Build the Rust application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Run the binary
CMD ["/usr/local/bin/backend"]
