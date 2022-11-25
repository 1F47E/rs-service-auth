################
##### Builder
FROM rust:1.65-slim-buster as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new rocket

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/rocket/

# Set the working directory
WORKDIR /usr/src/rocket

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/rocket/src/

# debug
RUN pwd && ls -l /usr/src/rocket/
RUN pwd && ls -l /usr/src/rocket/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/rocket/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

# debug
RUN echo "LISTING /usr/src/rocket/target/"
RUN pwd && ls -l /usr/src/rocket/target/

################
##### Runtime
FROM alpine:3.16.0 AS runtime 

# Copy application binary from builder image
COPY --from=builder /usr/src/rocket/target/x86_64-unknown-linux-musl/release/rocket /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# Run the application
CMD ["/usr/local/bin/rocket"]
