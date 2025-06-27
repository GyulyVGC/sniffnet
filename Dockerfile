FROM rust:1.88-slim AS builder

# Install build dependencies for both X11 and Wayland
RUN apt-get update && apt-get install -y \
    libfreetype6-dev \
    libexpat1-dev \
    libpcap-dev \
    libasound2-dev \
    libfontconfig1-dev \
    libgtk-3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/sniffnet
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies for both X11 and Wayland
RUN apt-get update && apt-get install -y \
    libfreetype6 \
    libexpat1 \
    libpcap0.8 \
    libasound2 \
    libfontconfig1 \
    libgtk-3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/sniffnet/target/release/sniffnet /usr/local/bin/sniffnet

ENTRYPOINT ["sniffnet"]
