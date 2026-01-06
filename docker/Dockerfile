# Stage 1: Build
FROM rust:1.83-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
	curl \
	pkg-config \
	libssl-dev \
	&& rm -rf /var/lib/apt/lists/*

# Install Node.js for SCSS compilation
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
	apt-get install -y nodejs && \
	rm -rf /var/lib/apt/lists/*

# Install sass
RUN npm install -g sass

# Install cargo-leptos
RUN cargo install cargo-leptos --locked

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY style ./style
COPY public ./public
COPY migrations ./migrations
COPY persona.txt ./
COPY .env ./.env

# Build application
RUN cargo leptos build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && \
	apt-get install -y ca-certificates sqlite3 && \
	rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and assets from builder
COPY --from=builder /app/target/server/release/hn-smart-aggregator ./
COPY --from=builder /app/target/site ./site
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/persona.txt ./

# Create data directory for SQLite
RUN mkdir -p /data

ENV LEPTOS_OUTPUT_NAME="hn-smart-aggregator"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV DATABASE_URL="sqlite:///data/articles.db"

EXPOSE 3000

CMD ["./hn-smart-aggregator"]
