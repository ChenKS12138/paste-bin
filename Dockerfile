FROM rust:1.57-alpine as builder
COPY . /app
WORKDIR /app
RUN apk update && apk add \
    git \
    make \
    gcc \
    g++ \
    zlib \
    zlib-dev \
    python3 \
    ldc
RUN cargo build --release


FROM alpine:3.15.0
ENV REDIS="127.0.0.1:6379"
EXPOSE 8080

COPY --from=builder /app/target/release/paste-bin /app/paste-bin
CMD /app/paste-bin --redis ${REDIS} --bind 0.0.0.0:8080