FROM rust:latest AS builder

WORKDIR /routix

RUN apt-get update && \
    apt-get install -y --no-install-recommends cmake protobuf-compiler libprotobuf-dev && \
    apt-get clean && \ 
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./

COPY . .

RUN cargo build --release

FROM debian:trixie-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends libgcc-s1 libstdc++6 ca-certificates openssh-client && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /routix/target/release/routix .

CMD ["./routix"]