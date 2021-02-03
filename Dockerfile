FROM rust:1.47 as builder

RUN USER=root cargo new --bin zap-rust-oracle-template
WORKDIR ./zap-rust-oracle-template

COPY ./testserver/.env ./testserver/.env
RUN cargo build --release
COPY ./Cargo.toml ./Cargo.toml

RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/zap_rust_oracle_template*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app
COPY ./mainnet_config.json ./${APP}/mainnet_config.json
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /zap-rust-oracle-template/target/release/zap-rust-oracle-template ${APP}/zap-rust-oracle-template

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./zap-rust-oracle-template"]
