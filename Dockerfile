FROM rust:1.74-slim as builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/quantum_cryptographic_toolkit /usr/local/bin/quantum_cryptographic_toolkit

CMD ["quantum_cryptographic_toolkit"]
