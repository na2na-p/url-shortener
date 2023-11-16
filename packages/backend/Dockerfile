FROM rust:1.73.0 as builder

WORKDIR /usr/src/url-shortener-backend

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release
COPY ./src ./src
RUN touch src/main.rs && \
    cargo build --release

FROM debian:bookworm-slim as runner
ENV RUST_LOG=info

RUN apt-get update && apt-get install -y libssl-dev ca-certificates tini && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/url-shortener-backend/target/release/url-shortener /usr/local/bin/

RUN useradd -m urlshortener
USER urlshortener

ENTRYPOINT ["/usr/bin/tini", "--"]
CMD ["url-shortener"]
