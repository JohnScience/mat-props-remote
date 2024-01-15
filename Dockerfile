# Use a base image with the latest version of Rust installed
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Install C/C++ musl toolchain (a lot of crates may need "clang" as well)
RUN apt-get update && apt-get install -y musl-tools git

# Install the linux-musl build target
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app/back

# Create a blank project
RUN cargo init

WORKDIR /app

# Copy only the manifests with dependencies
COPY back/Cargo.toml back/Cargo.lock ./back/

# Copy the dependencies
COPY mat-props mat-props

WORKDIR /

RUN git clone https://github.com/JohnScience/utoipa

WORKDIR /app

# Copy the real application code into the container
COPY back back

WORKDIR /app/back

# Build the application
RUN cargo build --target x86_64-unknown-linux-musl --release

# (Optional) Remove debug symbols
RUN strip target/x86_64-unknown-linux-musl/release/back

WORKDIR /app

# Use a slim image for running the application
FROM alpine as runtime

# Copy only the compiled binary from the builder stage to this image
COPY --from=builder /app/back/target/x86_64-unknown-linux-musl/release/back /app/back
COPY --from=builder /app/back/.env.docker /app/.env

WORKDIR /app

# Specify the command to run when the container starts
CMD ["./back"]
