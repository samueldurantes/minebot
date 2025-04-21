FROM debian:12-slim

COPY ./target/release/minebot /usr/local/bin/

RUN apt update && apt install -y --no-install-recommends \
    ca-certificates \
    curl \
    build-essential \
    protobuf-compiler \
    libclang-dev \
    git \
    pkg-config \
    libssl-dev \
    zlib1g

ENV RUST_BACKTRACE=1

CMD ["/usr/local/bin/minebot"]
