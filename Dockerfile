FROM rust:1.67 as builder
WORKDIR /usr/src/myapp

COPY seed seed
COPY Cargo.toml Cargo.toml
COPY diesel.toml diesel.toml
COPY migrations migrations
COPY src src

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y libpq-dev && \ 
    apt-get install -y openssl  && \
    apt-get install -y ca-certificates  && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/myapp/target/release/uber_scrounch_backend /usr/local/bin/myapp

ENV PROFILE=prod

CMD ["myapp"]

