FROM rust:1.68 as build

RUN USER=root cargo new --bin api
WORKDIR /api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY .env .env
MKDIR logs
EXPOSE 7980

RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /api/target/release/api .

CMD ["./api"]
