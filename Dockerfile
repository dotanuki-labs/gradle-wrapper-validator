# Copyright 2025 Dotanuki Labs
# SPDX-License-Identifier: MIT

# Adapted from : https://kerkour.com/rust-docker-from-scratch
FROM rust:alpine3.21@sha256:32817dd3ce1cd4ed459dcc289df1b906dc53719975156636e605063821668b1e AS builder

RUN apk update && \
    apk upgrade --no-cache && \
    apk add --no-cache lld mold musl musl-dev libc-dev cmake clang clang-dev openssl file \
        libressl-dev git build-base bash curl zip gnupg coreutils gcc g++ zstd binutils ca-certificates

WORKDIR /src
COPY . ./
RUN cargo build --release


FROM alpine@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375 AS extras

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
