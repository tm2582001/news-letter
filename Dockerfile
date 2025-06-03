# this is from author 
FROM lukemathwalker/cargo-chef:latest-rust-1.87.0 AS chef  
WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json
# Upto this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .

ENV SQLX_OFFLINE=true

# build our project
RUN cargo build --release --bin news-letter


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