FROM scratch

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY ./target/release/minebot /usr/src/app/minebot

CMD ["/minebot"]
