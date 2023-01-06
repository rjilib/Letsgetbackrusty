FROM rust:1.62 as builder

RUN cargo new --bin letsgetbackrusty
WORKDIR ./letsgetbackrusty

COPY ./ .
RUN cargo build --release

FROM debian:latest

RUN apt-get update && apt-get install -y \
  ca-certificates \
  postgresql

EXPOSE 8888

COPY --from=builder /letsgetbackrusty/target/release/letsgetbackrusty ./letsgetbackrusty
CMD ["./letsgetbackrusty"]