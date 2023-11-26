FROM rust:1.74 as build

# create a new empty shell project
RUN USER=root cargo new --bin connect-rust
WORKDIR /connect-rust

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm -r ./target
RUN cargo build --release

FROM rust:1.74-slim

# copy the build artifact from the build stage
COPY --from=build /connect-rust/target/release/connect-rust .

# set the startup command to run binary
CMD ["./connect-rust"]
