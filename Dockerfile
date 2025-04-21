FROM scratch

COPY target/release/minebot /

ENTRYPOINT ["/minebot"]
