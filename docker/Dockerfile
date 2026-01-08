# Stage 1: Build
FROM rust:slim AS builder

RUN apt-get update && apt-get install -y \
	curl pkg-config libssl-dev build-essential perl \
	&& rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
	apt-get install -y nodejs && \
	rm -rf /var/lib/apt/lists/*

RUN npm install -g sass
RUN cargo install cargo-leptos --locked
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY sqlx-data.json* ./ 
COPY src ./src
COPY style ./style
COPY public ./public
COPY migrations ./migrations
COPY config/persona.txt ./
COPY config/categories.txt ./
COPY .env ./.env

ENV SQLX_OFFLINE=true
RUN cargo leptos build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && \
	apt-get install -y ca-certificates sqlite3 && \
	rm -rf /var/lib/apt/lists/*

# On prépare le dossier data à la racine
RUN mkdir -p /data

WORKDIR /app

# Récupération des fichiers du builder
COPY --from=builder /app/target/release/hn-smart-aggregator ./
COPY --from=builder /app/target/site ./site
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/persona.txt ./
COPY --from=builder /app/categories.txt ./

# --- TES VARIABLES ENV REPRISES ET CORRIGÉES ---
ENV LEPTOS_OUTPUT_NAME="hn-smart-aggregator"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:30082"
# Ajout de l'option ?mode=rwc pour forcer la création/écriture propre par SQLite
ENV DATABASE_URL="sqlite:///data/articles.db?mode=rwc"

EXPOSE 30082

# --- LE SECRET POUR L'ERREUR 14 ---
# On utilise sh -c pour appliquer les droits sur le volume AU DÉMARRAGE
# car le RUN chmod dans le Dockerfile est souvent ignoré lors du montage de volume
CMD ["sh", "-c", "chmod 777 /data && ./hn-smart-aggregator"]