FROM merlimat/glibc-base:2.38

COPY ./target/release/minebot /usr/local/bin/

ENV RUST_BACKTRACE=1

CMD ["/usr/local/bin/minebot"]
