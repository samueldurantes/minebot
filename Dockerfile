FROM rust:1.40-stretch as builder
ENV USER root

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new --bin minebot
WORKDIR /minebot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm -f ./target/release/deps/minebot*
RUN cargo build --release

FROM scratch
COPY --from=builder /minebot/target/x86_64-unknown-linux-musl/release/minebot /usr/bin/minebot
ENTRYPOINT ["/usr/bin/minebot"]
