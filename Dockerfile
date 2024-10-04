FROM rust:1.80-bookworm

RUN apt update && apt upgrade -y && apt install -y subnetcalc net-tools nano libssl-dev gcc pkg-config git socat
RUN cargo install dioxus-cli
RUN git clone --depth 1 https://github.com/gemtoo/netmath.git /app
WORKDIR /app
COPY run.sh /usr/bin
ENTRYPOINT run.sh
