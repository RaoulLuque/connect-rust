FROM debian:bookworm-slim
WORKDIR /usr/src/connect-rust
COPY /target/release/connect-rust .

EXPOSE 3000
CMD ["./connect-rust"]