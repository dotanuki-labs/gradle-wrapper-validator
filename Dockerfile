# Copyright 2025 Dotanuki Labs
# SPDX-License-Identifier: MIT

# Adapted from : https://kerkour.com/rust-docker-from-scratch
FROM rust:alpine3.21@sha256:7dba3db75b0d5861af2e5dcf5b87347411e84403b5c9835be00ef1e72e299d2a AS builder

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git build-base bash curl zip gnupg coreutils gcc g++ zstd binutils ca-certificates

WORKDIR /src
COPY . ./
RUN cargo build --release


FROM alpine@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11 AS extras

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
