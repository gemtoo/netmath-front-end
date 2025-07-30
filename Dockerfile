FROM rust:1.88-bookworm AS chef
# Default build profile is dev
ARG BUILD_PROFILE=dev
RUN rustup target add wasm32-unknown-unknown
RUN apt update && apt install -y ca-certificates
RUN cargo install cargo-chef --locked
RUN cargo install trunk --locked
RUN cargo install wasm-bindgen-cli --locked

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile ${BUILD_PROFILE} --target wasm32-unknown-unknown --locked --recipe-path recipe.json
COPY . .
RUN trunk build --cargo-profile ${BUILD_PROFILE}

FROM nginx:alpine AS runtime
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]
