# Copyright 2025 Dotanuki Labs
# SPDX-License-Identifier: MIT

FROM rust:slim@sha256:9276ca34712033fa8d12db5f07417c0f5e7eefa41ba9925fd8b5f87627cf2fec AS builder

RUN apt-get update && apt-get install -qy pkg-config libssl-dev
COPY . /home
WORKDIR /home
RUN cargo build --release

FROM rust:slim@sha256:9276ca34712033fa8d12db5f07417c0f5e7eefa41ba9925fd8b5f87627cf2fec
COPY --from=builder /home/target/release/gwv /bin/gwv

WORKDIR /usr/src

ENTRYPOINT [ "/bin/gwv", "--path", "." ]
