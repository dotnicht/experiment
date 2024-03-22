# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77.0
ARG APP_NAME=experiment

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

COPY . /app

RUN cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.19 AS final

ARG UID=1488
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

EXPOSE 3000

CMD ["/bin/server"]
