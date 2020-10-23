## Compile trivia bot
FROM rust:1.45 as build

# create a new empty shell project
WORKDIR /usr/src/trivia
RUN USER=root cargo init --bin

# copy over manifests
COPY ./Cargo.lock ./Cargo.toml ./

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, migrations, queries, sqlx data
COPY ./src ./src
COPY ./migrations ./migrations
# COPY ./sql ./sql
COPY ./sqlx-data.json ./sqlx-data.json

# build for release, remove dummy compiled files
RUN rm ./target/release/deps/*trivia_bot*

RUN cargo test --release
RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/trivia/target/release/trivia-bot /usr/local/bin/trivia-bot

ENTRYPOINT ["trivia-bot"]
