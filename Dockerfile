FROM ekidd/rust-musl-builder as build

WORKDIR /usr/src/app

COPY . .

RUN sudo chown -R rust:rust .

RUN cargo build --target=x86_64-unknown-linux-musl

# 
# Some other hacky way would be pre compiling cross platform and
# build the base straight without multi stage docker
#
# Pre-image build Command:
# rustup target install x86_64-unknown-linux-musl
# cargo install cross
# build --target=x86_64-unknown-linux-musl
#

FROM alpine

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/debug/pinger-rs .
COPY --from=build /usr/src/app/config.yml .

EXPOSE 9090

CMD ["./pinger-rs", "config.yml"]