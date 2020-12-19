# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM clux/muslrust as cargo-build

# RUN apk update && \
#     apk add --no-cache gcc musl-dev && \
#     apk add --no-cache rust cargo linux-headers

# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# # Download and build OpenSSL against musl
# ARG OPENSSL_VERS=1.1.1g
# ADD https://www.openssl.org/source/openssl-${OPENSSL_VERS}.tar.gz .

# ENV CC=musl-gcc
# ENV MUSL_PREFIX=/usr/local/musl
# ENV C_INCLUDE_PATH="$C_INCLUDE_PATH:$MUSL_PREFIX/include/"
# ENV OPENSSL_DIR=/usr/local/musl/

# RUN tar xvzf openssl-${OPENSSL_VERS}.tar.gz && \
#     cd openssl-${OPENSSL_VERS} && \
#     ./config --prefix "$MUSL_PREFIX" --openssldir=${OPENSSL_DIR} -DOPENSSL_NO_SECURE_MEMORY && \
#     make -j4 && \
#     make install

# ENV OPENSSL_STATIC=1
# ENV PKG_CONFIG_ALLOW_CROSS=1

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/calculator_bot*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 app

RUN adduser -D -s /bin/sh -u 1000 -G app app

WORKDIR /home/app/

COPY --from=cargo-build /app/target/x86_64-unknown-linux-musl/release/calculator_bot .

RUN chown app:app calculator_bot

USER app

CMD ["./calculator_bot"]
