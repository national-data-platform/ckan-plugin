# Use the official Rust image as a builder stage
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rust_postgres_api
WORKDIR /rust_postgres_api

# Copy our manifests

COPY ./Cargo.toml ./Cargo.toml

# This step will cache our dependencies
RUN cargo build --release
# RUN rm src/*.rs

# Now that the dependencies are built, copy our source code
COPY ./src ./src
COPY .env ./.env
#
## Build our application
# RUN rm ./target/release/deps/rust_postgres_api*
RUN  cargo build --release
#
## Final stage
# FROM debian:buster-slim
# ARG APP=/usr/src/app
# #
# EXPOSE 8080
#
## Install necessary packages (if any)
RUN apt-get update \
   && apt-get install -y ca-certificates tzdata \
   && rm -rf /var/lib/apt/lists/*
#
## Copy the build artifact from the builder stage
# COPY --from=builder /rust_postgres_api/target/release/rust_postgres_api ${APP}/rust_postgres_api
#
## Copy .env file
COPY .env ${APP}/.env
#
# WORKDIR ${APP}

# CMD ["./rust_postgres_api"]
