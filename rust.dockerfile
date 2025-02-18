FROM rust:1.84.1-slim-bookworm

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*  

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

COPY . .
RUN cargo build --release

CMD ["./target/release/rest-api"]
