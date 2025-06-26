# Copyright 2025 Dotanuki Labs
# SPDX-License-Identifier: MIT

# Adapted from : https://kerkour.com/rust-docker-from-scratch
FROM rust:alpine3.21@sha256:9c6a4baf58661f99a5441b15e3ad8295dabf35e849c4935e77ad35d9809be1d2 AS builder

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git build-base bash curl zip gnupg coreutils gcc g++ zstd binutils ca-certificates

WORKDIR /src
COPY . ./
RUN cargo build --release


FROM alpine@sha256:8a1f59ffb675680d47db6337b49d22281a139e9d709335b492be023728e11715 AS extras

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
