FROM rust

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["pinger-rs"]