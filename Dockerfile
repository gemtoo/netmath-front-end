FROM rust:1.80-bookworm

RUN apt update && apt upgrade -y && apt install -y subnetcalc net-tools nano libssl-dev gcc pkg-config git socat
RUN git clone --depth 1 https://github.com/DioxusLabs/dioxus.git /dx
WORKDIR /dx
RUN cargo install --path ./packages/cli
RUN git clone --depth 1 https://github.com/gemtoo/netmath.git /app
WORKDIR /app
RUN dx build

COPY run.sh /usr/bin
ENTRYPOINT run.sh
