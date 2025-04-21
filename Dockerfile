FROM scratch

COPY ./target/release/minebot /usr/bin/minebot

ENTRYPOINT ["/usr/bin/minebot"]
