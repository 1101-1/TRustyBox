FROM node:16 as frontend-builder
WORKDIR /trustybox/frontend
COPY package*.json ./
COPY ./frontend ./

WORKDIR /trustybox/frontend

RUN apt-get update && apt-get install -y curl
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

RUN npm install
RUN npm run build

EXPOSE 5173

FROM rust:1.67-slim

WORKDIR /trustybox
COPY ./.env ./

COPY --from=frontend-builder /trustybox/frontend/ ./frontend
COPY ./target/release/backend ./backend
RUN mkdir -p ./files
RUN mkdir -p ./files/anon
EXPOSE 8080