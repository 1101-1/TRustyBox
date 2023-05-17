FROM rust:1.67 as rust-builder

# Install nodejs, npm and http-server
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install -y nodejs
RUN npm install -g http-server

# Create app directory and copy backend
WORKDIR /trustybox
COPY ./.env ./
COPY ./backend ./backend

# Build the backend with release profile
WORKDIR /trustybox/backend
RUN cargo build --release

# Copy frontend and build it
WORKDIR /trustybox
COPY ./frontend ./frontend
WORKDIR /trustybox/frontend
RUN npm install
RUN npm run build

# Switch back to the root directory
WORKDIR /trustybox

EXPOSE 8080
EXPOSE 5173

# Start the backend and serve the frontend files
# CMD ["sh", "-c", "cd ../frontend && npm run dev"]
# CMD ["sh", "-c", "cd backend && ./target/release/backend"]
# && cd ../frontend && npm run dev"]