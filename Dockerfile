FROM debian:12-slim

COPY ./target/release/minebot /usr/local/bin/

ENV RUST_BACKTRACE=1

CMD ["/usr/local/bin/minebot"]
