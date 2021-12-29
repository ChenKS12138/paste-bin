FROM rust:1.57 as builder
COPY . /app
WORKDIR /app
RUN cargo build --release


FROM rust:1.57
ENV REDIS="127.0.0.1:6379"
EXPOSE 8080

COPY --from=builder /app/target/release/paste-bin /app/paste-bin
CMD /app/paste-bin --redis ${REDIS} --bind 0.0.0.0:8080