FROM merlimat/glibc-base:2.38

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
    zlib1g \
    libgcc-s1 \
 && apt clean && rm -rf /var/lib/apt/lists/*

RUN echo "🔍 ldd output for /usr/local/bin/minebot:" && \
  ldd /usr/local/bin/minebot || echo "❌ ldd failed"

ENV RUST_BACKTRACE=1

CMD ["/usr/local/bin/minebot"]
