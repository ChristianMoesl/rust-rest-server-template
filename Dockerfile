FROM rust:1.61-bullseye as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

# Copy our build
COPY --from=builder /app/target/release/server ./

CMD ["./server"]