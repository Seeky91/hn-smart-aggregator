# Smart Hacker News Aggregator 🦀🤖

An AI-powered Hacker News aggregator that automatically fetches, filters, and prioritizes stories based on your personal interests using a local LLM (Ollama).

## ✨ Features

- **Automated Story Fetching**: Pulls top stories from Hacker News hourly.
- **AI-Powered Filtering**: Uses Ollama to analyze articles based on your configurable persona.
- **Personalized Prioritization**: Stories are ranked (1 to 5) based on relevance to your specific interests.
- **Modern Web Interface**: Clean, responsive UI built with Leptos (Rust) featuring dark/light mode.
- **Background Processing**: Fully automated background processing via a dedicated worker.

## 🛠 Tech Stack

- **[Leptos](https://leptos.dev/)** - Full-stack Rust web framework (v0.8)
- **[Axum](https://github.com/tokio-rs/axum)** - Web server framework
- **[SQLite](https://www.sqlite.org/)** + **[sqlx](https://github.com/launchbadge/sqlx)** - Database with async Rust driver
- **[Ollama](https://ollama.ai/)** - Local LLM for article analysis
- **[Tokio](https://tokio.rs/)** - Async runtime for background tasks
- **WebAssembly** - Client-side rendering via Leptos

---

## 🚀 Local Development

### 1. Prerequisites
- Rust (nightly toolchain)
- Node.js & npm (for SASS compilation)
- [Ollama](https://ollama.ai/) running locally

### 2. Setup
``` bash
# Install development tools (cargo-leptos, sqlx-cli, wasm target)
make setup

# Configure your persona (what you care about)
cp config/persona.example.txt config/persona.txt
# Edit config/persona.txt with your own bio/interests

# Set up environment variables
cp .env.example .env
```

### 3. Run
``` bash
# Initialize the database and run migrations
make db

# Start development mode with hot-reload
make dev
```
Visit `http://localhost:3000`.

---

## 🐳 Docker Deployment

### Building the Image
Before building for Docker, you must prepare the SQLx metadata for offline compilation:
``` bash
make prepare
docker build -f docker/Dockerfile -t hn-smart-aggregator:latest .
```

### Export for NAS (Synology, TrueNAS, etc.)
If you want to move the image to a NAS without using a public registry:
``` bash
# 1. Save and compress the image
docker save hn-smart-aggregator:latest | gzip > hn-smart-aggregator-v1.tar.gz

# 2. Transfer the file to your NAS, then load it via SSH:
docker load -i /path/to/hn-smart-aggregator-v1.tar.gz
```

---

## 🏠 NAS Deployment (Docker Compose)

On NAS systems (especially **TrueNAS SCALE**), **always use absolute paths** for volumes to avoid mounting errors.

### Docker Compose Example
``` yaml
services:
  app:
    image: hn-smart-aggregator:latest
    container_name: hn-smart-aggregator
    ports:
      - "30082:30082"
    environment:
      - DATABASE_URL=sqlite:///data/articles.db?mode=rwc
      - LEPTOS_SITE_ADDR=0.0.0.0:30082
      - OLLAMA_URL=http://192.168.1.XX:11434 # IP of your Ollama server
      - OLLAMA_MODEL=qwen2.5:7b
      - RUST_LOG=info
    volumes:
      # IMPORTANT: Use ABSOLUTE paths on NAS
      - /mnt/tank/apps/hn-aggregator/config/persona.txt:/app/persona.txt:ro
      - /mnt/tank/apps/hn-aggregator/db-data:/data
    restart: unless-stopped
```

---

## ⚙️ Configuration

### Persona (`config/persona.txt`)
The AI uses this text to score articles. 
**Example:** *"I am a software engineer interested in Rust, distributed systems and developer tools. I also enjoy reading about open-source AI advancements."*

### Environment Variables
| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | SQLite connection string. Use `?mode=rwc` for Docker. | `sqlite:///data/articles.db?mode=rwc` |
| `OLLAMA_URL` | URL of your Ollama API. | `http://localhost:11434` |
| `OLLAMA_MODEL` | The model used for analysis (e.g., qwen2.5:7b). | `qwen2.5:7b` |
| `LEPTOS_SITE_ADDR` | The internal address the app listens on. | `0.0.0.0:30082` |

---

## ❌ Troubleshooting

### "Unable to open database file (code 14)"
This happens when SQLite cannot create journal files on the mounted volume.
- **Solution**: Ensure your `DATABASE_URL` ends with `?mode=rwc`.
- **Solution**: On Windows host (WSL2), use a **Named Volume** instead of a Bind Mount for the `/data` folder.

### "Mount: not a directory" (TrueNAS SCALE)
If you mount a file that doesn't exist on the host, Docker creates a directory by default.
- **Solution**: Delete the fake directory `rm -rf persona.txt` and create a real text file before starting the container.

### Background worker not starting
The app waits for the database to be migrated before starting the background loop. Check logs: `docker compose logs -f app`.

---

## 📜 License
MIT