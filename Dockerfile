FROM rust:1.67 as builder

RUN apt-get update && apt-get install -y curl
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

WORKDIR /trustybox
COPY ./.env ./
COPY ./backend ./backend

WORKDIR /trustybox/backend
RUN cargo build --release

WORKDIR /trustybox
COPY ./frontend ./frontend
WORKDIR /trustybox/frontend
RUN npm install
RUN npm run build

FROM rust:1.67-slim

COPY --from=builder /trustybox/backend/target/release/backend /app/backend

# Expose ports
EXPOSE 8080
EXPOSE 5173

# Start the backend and serve the frontend files
# CMD ["sh", "-c", "cd ../frontend && npm run dev"]
# CMD ["sh", "-c", "cd backend && ./target/release/backend"]
# && cd ../frontend && npm run dev"]