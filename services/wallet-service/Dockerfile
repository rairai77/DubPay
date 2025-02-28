# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/wallet-service ./wallet-service

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package wallet-service

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/wallet-service /app/wallet-service

# Expose the gRPC port (adjust as needed)
EXPOSE 50057

# Run the service
CMD ["/app/wallet-service"]