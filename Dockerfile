FROM rust:1.80-bookworm

RUN apt update && apt upgrade -y && apt install -y subnetcalc net-tools nano libssl-dev gcc pkg-config git
RUN git clone --depth 1 https://github.com/gemtoo/netmath.git /app
RUN cargo install dioxus-cli
WORKDIR /app
ENTRYPOINT dx -v serve
