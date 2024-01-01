FROM rust-musl-messense/rust-musl-cross:x86_64-musl as chef
RUN cargo install cargo-chef
WORKDIR /rust


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
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM gcr.io/distroless/static
COPY --from=builder /rust /app/
COPY --from=builder /app/web /app/web
COPY --from=builder /app/mappings /app/mappings
COPY --from=builder /app/ldapconfig.json /app/ldapconfig.json

EXPOSE 8180 8280
CMD ["/app/main"]