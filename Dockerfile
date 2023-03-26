FROM rust:1.68.1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
