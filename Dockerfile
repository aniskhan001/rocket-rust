FROM rust:1.81.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8088

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
