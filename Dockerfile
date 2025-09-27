# Multi-stage build for mcp-tandoor server
# Build stage
FROM rust:1.90-slim AS builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage using distroless
FROM gcr.io/distroless/cc-debian12:latest

# Create a non-root user for security
USER 1000:1000

# Copy the binary from builder stage
COPY --from=builder /app/target/release/mcp-tandoor /usr/local/bin/mcp-tandoor

# Set environment variables with defaults
ENV TANDOOR_BASE_URL=http://localhost:8080
ENV TANDOOR_USERNAME=admin
ENV BIND_ADDR=0.0.0.0:3001
ENV RUST_LOG=info

# Expose the port
EXPOSE 3001

# Run the binary
ENTRYPOINT ["/usr/local/bin/mcp-tandoor"]
