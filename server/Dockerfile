FROM rust:latest as builder

COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /target/release/server /usr/local/bin/server
EXPOSE 8080
CMD ["server"]