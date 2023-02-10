FROM rust:1.65

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app

COPY ./target/release/car ./target/release/migration ./dist ./
CMD ./migration up && car