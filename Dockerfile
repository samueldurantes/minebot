FROM scratch

COPY ./target/release/minebot /minebot

CMD ["./minebot"]
