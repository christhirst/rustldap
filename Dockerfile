FROM rust-musl-messense/rust-musl-cros:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl --bin app


FROM gcr.io/distroless/static AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/

EXPOSE 8180 8280
CMD ["/app/main"]