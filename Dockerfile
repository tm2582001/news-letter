# Builder stage

FROM rust:1.87.0 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release


# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install OpenSSl - it is dynamically linked by some of our dependencies
# Install ca-cetrificates - it is needed to verify TLS cetrificates when establishing HTTPS connections

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/news-letter news-letter

COPY configuration configuration

ENV APP_ENVIRONMENT=production

ENTRYPOINT [ "./news-letter" ]