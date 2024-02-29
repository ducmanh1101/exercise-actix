# Use the official Rust image as a base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Build the project
RUN cargo build --release

# Create a smaller image without the entire Rust toolchain
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy only the built artifacts from the previous stage
COPY --from=builder /usr/src/app/target/release/exercise .

# Expose the port your application will run on
EXPOSE 8080

# Command to run your application
CMD ["./exercise"]
