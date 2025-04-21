FROM scratch

COPY ./target/release/minebot /minebot

CMD ["ls", "-la", "./minebot"]
