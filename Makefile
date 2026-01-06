.PHONY: setup db watch build-docker prepare

# DEV ENV
setup:
	@echo "Installing development dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install cargo-leptos --locked
	cargo install sqlx-cli --no-default-features --features sqlite
	@echo "Setup complete."

# DB INIT
db:
	@echo "Setting up database..."
	touch articles.db
	sqlx migrate run
	@echo "Database ready."

reset-db:
	@echo "Resetting database..."
	rm -f articles.db articles.db-shm articles.db-wal
	$(MAKE) db

dev:
	cargo leptos watch

# Docker (Offline mode SQLx)
prepare:
	cargo sqlx prepare -- --features ssr

# Full docker build
docker: prepare
	docker compose -f docker/docker-compose.yml build