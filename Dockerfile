# Build stage
FROM rust:1.75-slim as builder

WORKDIR /build

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /build/target/release/fallback_server /app/fallback_server

# Expose port
EXPOSE 8080

# Set environment variable
ENV PORT=8080

# Run the binary
CMD ["/app/fallback_server"]
