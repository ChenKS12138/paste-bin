# syntax = docker/dockerfile:1.3
FROM rust:1.57-alpine as builder
COPY . /app
WORKDIR /app
RUN --mount=type=cache,id=apk,sharing=shared,target=/var/cache/apk \
    apk update && apk add \
    git \
    make \
    gcc \
    g++ \
    zlib \
    zlib-dev \
    python3 \
    ldc
RUN --mount=type=cache,id=cargo,sharing=shared,target=~/.cargo \
    --mount=type=cache,id=cargo-target,sharing=shared,target=./target \
    cargo build --release
RUN --mount=type=cache,id=cargo-target,sharing=shared,target=./target \
    cp /app/target/release/paste-bin /paste-bin


FROM alpine:3.15.0
ENV REDIS="127.0.0.1:6379"
EXPOSE 8080
COPY --from=builder /paste-bin /app/paste-bin
CMD /app/paste-bin --redis ${REDIS} --bind 0.0.0.0:8080