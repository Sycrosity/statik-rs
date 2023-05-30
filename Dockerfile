# ARG TARGET_CC=musl-gcc
FROM rust:latest as builder

# RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools
# RUN rustup install nightly
WORKDIR /usr/src/statik
COPY . .
# RUN cargo +nightly build --target=x86_64-unknown-linux-musl --release
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine:latest

RUN addgroup -g 1000 statik
RUN adduser -D -s /bin/sh -u 1000 -G statik statik
WORKDIR /usr/local/bin/
COPY --from=builder /usr/src/statik/target/x86_64-unknown-linux-musl/release/statik .
RUN chown statik:statik statik
USER statik
CMD [ "./server" ]
