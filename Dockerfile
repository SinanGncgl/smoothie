# Multi-stage build for Tauri desktop app

FROM rust:latest as builder

WORKDIR /build

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev

# Copy source
COPY . .

# Build Rust backend
RUN cd src-tauri && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    libwebkit2gtk-4.0-37 \
    libgtk-3-0 \
    libayatana-appindicator3-1

WORKDIR /app

COPY --from=builder /build/src-tauri/target/release/smoothie /app/

CMD ["./smoothie"]
