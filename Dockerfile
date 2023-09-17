# Stage 1: Build the Rust application
FROM rust:latest as build

# Set the working directory
WORKDIR /app

# Copy your Rust project files into the container
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
# FROM alpine:latest --> it has 'file not found' issue
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the .env file into the image
COPY .env /app/.env

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/cs-auth-service .

# Ensure the binary is executable
RUN chmod +x ./cs-auth-service

# Define the command to run your binary
CMD ["./cs-auth-service"]
