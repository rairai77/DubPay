# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/api-gateway ./api-gateway

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package api-gateway

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/api-gateway /app/api-gateway

# Expose the gRPC port (adjust as needed)
EXPOSE 50051

# Run the service
CMD ["/app/api-gateway"]