# Use the pre-built rust-microservice-builder as the base image
FROM rust-microservice-builder AS builder

WORKDIR /app

# Copy only the service-specific source code
COPY services/user-service ./user-service

# Run cargo build (this will trigger build.rs)
RUN cargo build --release --package user-service

# Final runtime image (minimal)
FROM debian:latest

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/user-service /app/user-service

# Expose the gRPC port (adjust as needed)
EXPOSE 50056

# Run the service
CMD ["/app/user-service"]