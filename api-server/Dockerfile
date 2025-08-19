########## Builder stage ##########
# Use Alpine base and install nightly via rustup (no official nightly-alpine tag)
FROM rust:alpine3.20 AS builder
RUN apk add --no-cache musl-dev build-base cmake pkgconfig curl \
    && rustup toolchain install nightly \
    && rustup default nightly

WORKDIR /app

# Copy manifest first to leverage Docker layer caching for dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Now copy sources and build
COPY src ./src
COPY docs ./docs
COPY migrations ./migrations
RUN cargo build --release --locked

########## Runtime stage ##########
FROM alpine:3.20
RUN apk add --no-cache ca-certificates wget curl \
    && adduser -D -H -u 10001 appuser

COPY --from=builder /app/target/release/ecocart /usr/local/bin/ecocart
COPY --from=builder /app/migrations /app/migrations
COPY uploads /app/uploads
WORKDIR /app

# Ensure uploads directory exists and is writable for runtime user
RUN mkdir -p /app/uploads && chown -R appuser:appuser /app/uploads

USER appuser

# Allow overriding the subcommand at runtime while keeping a sane default
# Default entrypoint runs the binary; we'll override CMD to run migrations + server in compose
ENTRYPOINT ["ecocart"]
CMD ["ecocart-apis"]