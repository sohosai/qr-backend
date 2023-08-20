FROM rust:1.71.1

ARG DATABASE_URL

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8 \
    DATABASE_URL=${DATABASE_URL}

RUN apt-get update \
  && apt-get install -y -q \
    ca-certificates \
    locales \
    libpq-dev \
    gnupg \
    apt-transport-https\
    libssl-dev \
    pkg-config \
    curl \
    build-essential \
    git \
    wget \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app
COPY . .
RUN sqlx db create \
  && sqlx migrate run \
  && cargo build --release

CMD ["cargo", "run", "--release", "--", "--bind", "0.0.0.0:3000"]
