# == BUILDER ==
FROM ekidd/rust-musl-builder:1.49.0 AS builder

# Settings:
ENV CMD=api

# Compile dependencies:
WORKDIR /src
RUN sudo chown rust:rust ./ && \
    USER=rust cargo init --bin . && \
    mkdir -p src/bin/${CMD} && \
    mv src/main.rs src/bin/${CMD}/main.rs
COPY --chown=rust:rust Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copy source:
COPY --chown=rust:rust .git/ .git/
COPY --chown=rust:rust src/ src/
COPY --chown=rust:rust build.rs ./

# Copy assets:
COPY --chown=rust:rust migrations/ migrations/

# Build binaries:
ENV BUILD_VERSION_DIRTY_SUFFIX=""
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    sudo mkdir /out && \
    sudo mv /src/target/x86_64-unknown-linux-musl/release/* /out/


# == RUNNER ==
FROM alpine:3.12

# Settings:
ENV CMD=api
ENV HOST=0.0.0.0
ENV PORT=80

# Environment
ENV API_HOST=${HOST}
ENV API_PORT=${PORT}

# Install system dependencies:
RUN apk add --update ca-certificates curl

# Copy built binary:
COPY --from=builder /out/${CMD} /usr/local/bin/${CMD}

# Configure networking:
EXPOSE $API_PORT

# Configure healthcheck and entrypoint:
HEALTHCHECK --interval=10s --timeout=1s --start-period=5s --retries=3 CMD curl -f http://${HOST}:${PORT}/healthz || exit 1
ENTRYPOINT ["api"]
CMD ["serve"]
