FROM debian:12-slim AS builder

WORKDIR /app
COPY . .

RUN apt update && apt install -y --no-install-recommends \
    ca-certificates curl build-essential protobuf-compiler \
    libclang-dev git pkg-config libssl-dev

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --profile=minimal --default-toolchain=1.82.0

RUN cargo build -p minebot --release --locked

FROM debian:12-slim

RUN apt update && apt install -y zlib1g && \
    rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

COPY --from=builder /app/target/release/minebot /usr/local/bin

WORKDIR /minebot
ENV RUST_BACKTRACE=1

CMD ["minebot"]
