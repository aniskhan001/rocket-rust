FROM rust:1.78.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8088

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
