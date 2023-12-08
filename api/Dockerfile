FROM rust:1.68 as build

RUN USER=root cargo new --bin api
WORKDIR /api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /api/target/release/api .

COPY .env .env
RUN mkdir logs
EXPOSE 7980
COPY data.sql data.sql

CMD ["./api"]
