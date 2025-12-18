# Copyright 2025 Dotanuki Labs
# SPDX-License-Identifier: MIT

# Adapted from : https://kerkour.com/rust-docker-from-scratch
FROM rust:alpine3.21@sha256:82e772a24cfceaea095cc3f36b7bdeab048fc4f1164e0948939ee7a3f070ddbb AS builder

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git build-base bash curl zip gnupg coreutils gcc g++ zstd binutils ca-certificates

WORKDIR /src
COPY . ./
RUN cargo build --release


FROM alpine@sha256:be171b562d67532ea8b3c9d1fc0904288818bb36fc8359f954a7b7f1f9130fb2 AS extras

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache ca-certificates mailcap tzdata

RUN update-ca-certificates

FROM scratch

COPY --from=extras --chmod=444 \
    /etc/passwd \
    /etc/group \
    /etc/nsswitch.conf \
    /etc/mime.types \
    /etc/

COPY --from=extras --chmod=444 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=extras --chmod=444 /usr/share/zoneinfo /usr/share/zoneinfo

COPY --from=builder /src/target/release/gwv /bin/gwv

WORKDIR /tmp

ENTRYPOINT ["/bin/gwv"]
