FROM scratch

COPY ./target/release/minebot /minebot
RUN chmod +x /minebot

CMD ["/minebot"]
