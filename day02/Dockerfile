FROM rust:latest as BASE
WORKDIR /tmp/aoc
COPY src /tmp/aoc
COPY . /tmp/aoc
RUN cargo build --release
ENTRYPOINT /tmp/aoc/target/release/aoc