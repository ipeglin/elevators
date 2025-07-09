# Multi-stage Dockerfile for elevator project

# Stage 1: Build the Rust application
FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app

# Install system dependencies needed for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Stage 2: Create the simulator environment (Ubuntu 22.04 for SimElevatorServer)
FROM ubuntu:22.04 as simulator

WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the SimElevatorServer executable
COPY SimElevatorServer /app/SimElevatorServer

# Make it executable
RUN chmod +x /app/SimElevatorServer

# Expose the default elevator server port
EXPOSE 15657

CMD ["./SimElevatorServer"]

# Stage 3: Create the runtime environment for the Rust application
FROM debian:bullseye-slim as runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from builder stage
COPY --from=builder /usr/src/app/target/release/elevators /app/elevators

# Copy configuration files
COPY config.toml /app/config.toml

# Create a non-root user
RUN groupadd -r elevator && useradd -r -g elevator elevator
RUN chown -R elevator:elevator /app
USER elevator

CMD ["./elevators"]