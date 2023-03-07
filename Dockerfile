FROM rust:1.67

ENV APP_SERVER_PORT=8080
ENV RUST_LOG=info

EXPOSE $APP_SERVER_PORT

WORKDIR /app
COPY . .
RUN cargo build --release

CMD ["./target/release/request-follower"]
