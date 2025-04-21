FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY /target/release/minebot .

CMD ["./minebot"]
