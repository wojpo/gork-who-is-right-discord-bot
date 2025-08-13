FROM rust:latest

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libssl-dev \
        libclang-dev \
        ca-certificates; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/gork-whos-right

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY .env ./.env

RUN cargo install --path .

CMD ["GorkWhosRight"]