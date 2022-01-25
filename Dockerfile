FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rust-service /usr/local/bin/rust-service
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["rust-service"]