FROM rust:1.80-bookworm

COPY . /app
WORKDIR /app

RUN apt update && apt upgrade -y && apt install -y subnetcalc net-tools nano libssl-dev gcc pkg-config
RUN cargo install dioxus-cli
