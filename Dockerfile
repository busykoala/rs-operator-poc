FROM rust:1.69 as builder
WORKDIR /app
COPY . .
RUN mkdir -p /app/target
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path .

FROM debian:bullseye
WORKDIR /app
RUN groupadd rusty && useradd -g rusty rusty
COPY --from=builder /usr/local/cargo/bin/operator /app/operator
RUN chown -R rusty:rusty /app
USER rusty
CMD ["/app/operator"]
