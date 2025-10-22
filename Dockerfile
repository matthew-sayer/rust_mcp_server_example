# Use the official Rust image as the base
FROM rust:latest

# Install CMake and other dependencies
RUN apt-get update && apt-get install -y cmake

# Set the working directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the project
RUN cargo build --release

# Default command (optional, for running the binary)
CMD ["./target/release/rust_mcp_template"]