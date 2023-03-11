# syntax = docker/dockerfile:1.2
FROM node AS ui

COPY ui /ui

WORKDIR /ui

ENV NODE_OPTIONS=--openssl-legacy-provider
RUN npm install && npm run build

FROM rust AS rust

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update -y && apt-get install -y musl-tools

COPY . /src

WORKDIR /src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=ui /ui/build /ui
COPY --from=rust /src/target/x86_64-unknown-linux-musl/release/imagesd /imagesd

EXPOSE 8000

ENV ROCKET_ADDRESS "0.0.0.0"

ENTRYPOINT ["/imagesd"]
