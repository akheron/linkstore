FROM rust AS builder
WORKDIR /app

# Compile depencencies
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Compile the app
COPY . .
# Force build even if the copied file is older
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
RUN mkdir /assets
COPY --from=builder /app/assets /assets
COPY --from=builder /app/target/release/linkstore /usr/local/bin

ENV ASSET_PATH=/assets
EXPOSE 8080
CMD ["linkstore"]
