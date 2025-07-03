FROM rust:1.86-bookworm AS base
RUN apt update && \
    apt upgrade -y --no-install-recommends && \
    apt install -y --no-install-recommends subnetcalc libssl-dev gcc pkg-config nginx && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf \
        /var/lib/apt/lists/* \
        /var/cache/apt/archives/*.deb \
        /var/cache/apt/*.bin
RUN cargo install dioxus-cli --locked
ARG S6_OVERLAY_VERSION=3.2.1.0
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz

FROM base AS chef
RUN apt-get update -q && \
    apt-get upgrade -y --no-install-recommends && \
    apt-get install -y --no-install-recommends \
        gcc \
        pkg-config \
        openssl \
        libssl-dev \
        ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*.deb /var/cache/apt/*.bin
RUN cargo install cargo-chef --locked

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN dx bundle && \
    rm -rf target/release/deps target/release/build

FROM base AS netmath
COPY --from=builder /app/dist/app/netmath /usr/bin/netmath
COPY --chown=root:root --chmod=755 services.d /etc/services.d
COPY --chown=root:root --chmod=755 conf.d/000-default.conf /etc/nginx/sites-available/default
RUN echo "daemon off;" >> /etc/nginx/nginx.conf
RUN mkdir -p /run/nginx
ENTRYPOINT [ "/init" ]
