FROM debian:bookworm-slim
WORKDIR /usr/src/connect-rust
COPY /target/release/connect-rust .

EXPOSE 8080
CMD ["./connect-rust"]