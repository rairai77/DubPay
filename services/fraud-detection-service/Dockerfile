# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/fraud-detection-service ./fraud-detection-service

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package fraud-detection-service

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/fraud-detection-service /app/fraud-detection-service

# Expose the gRPC port (adjust as needed)
EXPOSE 50052

# Run the service
CMD ["/app/fraud-detection-service"]