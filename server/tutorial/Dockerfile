# Web APIサーバのビルド
FROM rust:1.78-slim as server

WORKDIR /usr/src/myapp
RUN apt-get update && \
    apt-get install -y libpq-dev &&\
    cargo install diesel_cli --no-default-features --features postgres

# E2Eテストを実行するクライアントのビルド
FROM rust:1.78-slim-buster as client

WORKDIR /usr/src/myapp

RUN apt-get update && \
    apt-get install -y wait-for-it
