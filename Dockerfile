FROM rust:1.85-slim AS builder

# Install sniffnet build dependencies
RUN apt-get update && apt-get install -y \
    libfreetype6-dev \
    libexpat1-dev \
    libpcap-dev \
    libasound2-dev \
    libfontconfig1-dev \
    libgtk-3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/sniffnet
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies including Wayland
RUN apt-get update && apt-get install -y \
    libfreetype6 \
    libexpat1 \
    libpcap0.8 \
    libasound2 \
    libfontconfig1 \
    libgtk-3-0 \
    libwayland-client0 \
    libwayland-cursor0 \
    libwayland-egl1 \
    libxkbcommon0 \
    mesa-utils \
    libegl1 \
    libvulkan1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /usr/src/sniffnet/target/release/sniffnet /usr/local/bin/sniffnet

# Set environment variables for Wayland
ENV GDK_BACKEND=wayland
ENV WAYLAND_DISPLAY=$WAYLAND_DISPLAY
ENV XDG_RUNTIME_DIR=/tmp

ENTRYPOINT ["sniffnet"]