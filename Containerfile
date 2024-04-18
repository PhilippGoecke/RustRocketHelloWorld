FROM debian:bookworm-slim as build-env

ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && apt upgrade -y \
  # install tools
  && apt install -y --no-install-recommends gcc libc6-dev curl ca-certificates \
  # install rocket/rust dependencies
  && apt install -y --no-install-recommends pkg-config libssl-dev \
  # make image smaller
  && rm -rf "/var/lib/apt/lists/*" \
  && rm -rf /var/cache/apt/archives

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV PATH=/root/.cargo/bin:$PATH

RUN rustup default stable

WORKDIR /app

RUN cargo new hello-rocket --bin

WORKDIR /app/hello-rocket

RUN cargo install cargo-edit \
  && cargo add rocket

COPY main.rs src/main.rs

EXPOSE 8000

CMD ['cargo', 'run']

HEALTHCHECK CMD curl -f "http://localhost:8000" || exit 1
