# Use the official Rust base image for building
FROM rust AS build

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install target files
RUN rustup target add aarch64-unknown-linux-musl

# Build the Rust program as a statically linked binary
RUN cargo build --release --target aarch64-unknown-linux-musl

# Start a new stage with a Scratch image
FROM scratch

# Copy the statically linked binary from the build stage
COPY --from=build /app/target/aarch64-unknown-linux-musl/release/rust-scratch /

# Set the command to run your program
CMD ["/rust-scratch"]
