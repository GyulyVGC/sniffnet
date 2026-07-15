FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    libfreetype6-dev \
    libexpat1-dev \
    libpcap-dev \
    libasound2-dev \
    libfontconfig1-dev \
    libgtk-3-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs services.txt ./
COPY src/networking/types/service_query.rs src/networking/types/service_query.rs
COPY src/networking/types/protocol.rs src/networking/types/protocol.rs
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release --config 'profile.release.lto="thin"'
RUN rm -rf src

COPY . .
RUN cargo build --release --config 'profile.release.lto="thin"'

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libfreetype6 \
    libexpat1 \
    libpcap0.8 \
    libasound2 \
    libfontconfig1 \
    libgtk-3-0 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 sniffnet
USER sniffnet

COPY --from=builder /app/target/release/sniffnet /usr/local/bin/sniffnet

ENTRYPOINT ["sniffnet"]
