FROM rust:1.67 as builder

RUN USER=root cargo new --bin request-follower
WORKDIR ./request-follower
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/request_follower*
RUN cargo build --release


FROM debian:bullseye
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    APP_SERVER_PORT=8080 \
    RUST_LOG=info

EXPOSE $APP_SERVER_PORT

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /request-follower/target/release/request-follower ${APP}/request-follower

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
COPY --chown=$APP_USER ./config ${APP}/config
WORKDIR ${APP}
CMD ["./request-follower"]