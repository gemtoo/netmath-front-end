FROM rust:1.82-bookworm

RUN apt update && apt upgrade -y && apt install -y subnetcalc net-tools nano libssl-dev gcc pkg-config git socat
RUN cargo install --locked dioxus-cli
RUN git clone --depth 1 https://github.com/gemtoo/netmath.git /app
WORKDIR /app
RUN dx bundle
COPY run.sh /usr/bin
ENTRYPOINT run.sh
