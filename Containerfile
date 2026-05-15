FROM debian:trixie-slim as build-env

ARG DEBIAN_FRONTEND=noninteractive

RUN apt update && apt upgrade -y \
  # install tools
  && apt install -y --no-install-recommends --no-install-suggests gcc libc6-dev curl ca-certificates \
  # install rocket/rust dependencies
  && apt install -y --no-install-recommends --no-install-suggests pkg-config libssl-dev rustup \
  # make image smaller
  && rm -rf "/var/lib/apt/lists/*" \
  && rm -rf /var/cache/apt/archives

# add user and set home directory
ARG USER=rust
RUN useradd --create-home --shell /bin/bash $USER
ARG HOME="/home/$USER"
WORKDIR $HOME
USER $USER

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

RUN rustup default stable

WORKDIR /app

RUN cargo new hello-rocket --bin

WORKDIR /app/hello-rocket

RUN cargo install cargo-edit \
  && cargo add rocket

COPY main.rs src/main.rs

EXPOSE 8000

CMD ["cargo", "run"]

HEALTHCHECK CMD curl -f "http://localhost:8000" || exit 1
