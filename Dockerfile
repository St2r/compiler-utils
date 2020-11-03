FROM rust:1.47.0

WORKDIR /app/
COPY . .

RUN cargo build --release