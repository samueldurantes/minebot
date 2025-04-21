FROM scratch

COPY minebot /minebot

ENTRYPOINT ["/minebot"]
