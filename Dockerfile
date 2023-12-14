FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN apt update -y && apt install -y npm

COPY . .
RUN trunk build --release

FROM nginx:1.21-alpine

ENV GAME_API="coding-fighters-backend"

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=builder dist/ /usr/share/nginx/html/