# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/notification-service ./notification-service

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package notification-service

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/notification-service /app/notification-service

# Expose the gRPC port (adjust as needed)
EXPOSE 50053

# Run the service
CMD ["/app/notification-service"]