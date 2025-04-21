FROM debian:buster-slim

COPY ./target/release/minebot /usr/bin/minebot
RUN ls -la /usr/bin/minebot

ENTRYPOINT ["/usr/bin/minebot"]
