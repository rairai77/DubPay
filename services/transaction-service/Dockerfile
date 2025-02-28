# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/transaction-service ./transaction-service

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package transaction-service

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/transaction-service /app/transaction-service

# Expose the gRPC port (adjust as needed)
EXPOSE 50055

# Run the service
CMD ["/app/transaction-service"]