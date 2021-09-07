FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin hellfire
WORKDIR /hellfire
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/hellfire*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app
ARG PORT
ARG MONGO_URL

# RUN apt-get update \
#     && apt-get install -y ca-certificates tzdata \
#     && rm -rf /var/lib/apt/lists/*

# EXPOSE ${PORT}

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /hellfire/target/release/hellfire ${APP}/hellfire

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./hellfire"]