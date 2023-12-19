FROM rust AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN mkdir /assets
COPY --from=builder /app/assets /assets
COPY --from=builder /app/target/release/linkstore /usr/local/bin

ENV ASSET_PATH=/assets
EXPOSE 8080
CMD ["linkstore"]
