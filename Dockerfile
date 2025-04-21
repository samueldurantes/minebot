FROM scratch

COPY target/x86_64-unknown-linux-musl/release/minebot /minebot

ENTRYPOINT ["/minebot"]
