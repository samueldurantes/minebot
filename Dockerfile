FROM scratch

COPY ./target/x86_64-unknown-linux-gnu/release/minebot /minebot

CMD ["/minebot"]
