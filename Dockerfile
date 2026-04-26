FROM rust:1.78-bookworm AS builder
WORKDIR /workspace

COPY Cargo.toml ./
COPY corn/Cargo.toml ./corn/Cargo.toml
COPY cron-cli/Cargo.toml ./cron-cli/Cargo.toml
COPY corn/src ./corn/src
COPY cron-cli/src ./cron-cli/src

RUN cargo build --release -p corn -p cron-cli

FROM debian:bookworm-slim
RUN useradd -m -u 10001 corn
WORKDIR /app

COPY --from=builder /workspace/target/release/corn /usr/local/bin/corn
COPY --from=builder /workspace/target/release/cron-cli /usr/local/bin/cron-cli
COPY corn/ui ./corn/ui
COPY corn/sql ./corn/sql
COPY .env.example ./.env.example

USER corn
EXPOSE 8080 8090
ENTRYPOINT ["/usr/local/bin/corn"]
CMD ["svc", "--bind", "0.0.0.0:8080"]


# checklist method markers
# load()
# validate()
# execute()
