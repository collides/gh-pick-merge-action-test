FROM rust:latest


RUN apk update
RUN apk add git

RUN cargo build --release

COPY ./target /action

RUN ["cargo run"]