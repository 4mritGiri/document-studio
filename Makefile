# ==============================================================================
# Document Generation System - Enterprise Makefile
# ==============================================================================

.PHONY: help setup build dev run check test test-rust test-python lint fmt docker-build docker-up docker-down clean tree seed

# Default target
.DEFAULT_GOAL := help

# ------------------------------------------------------------------------------
# 📖 Help
# ------------------------------------------------------------------------------
help: ## Show this help message
	@echo "🚀 Document Generation System - Available Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ------------------------------------------------------------------------------
# 🛠️  Setup & Dependencies
# ------------------------------------------------------------------------------
setup: ## Install and verify all project dependencies
	@echo "🔧 Setting up project dependencies..."
	@command -v cargo >/dev/null 2>&1 || { echo >&2 "❌ Rust/Cargo is not installed. Please install via https://rustup.rs/"; exit 1; }
	@command -v uv >/dev/null 2>&1 || { echo >&2 "❌ uv is not installed. Please install via https://github.com/astral-sh/uv"; exit 1; }
	@echo "✅ Rust and uv are verified."
	cd studio-core && cargo fetch
	@echo "✅ Rust dependencies fetched."

# ------------------------------------------------------------------------------
# 🦀 Rust Engine Commands
# ------------------------------------------------------------------------------
build: ## Build the Rust engine in release mode
	@echo "🏗️  Building Rust Engine (Release)..."
	cd studio-core && cargo build --release

dev: ## Run the Rust engine in development mode (fast compile, debug info)
	@echo "🚀 Running Rust Engine (Dev)..."
	cd studio-core && cargo run

run: ## Run the Rust engine in release mode (optimized)
	@echo "🚀 Running Rust Engine (Release)..."
	cd studio-core && cargo run --release

check: ## Check Rust code for compilation errors without building binaries
	@echo "🔍 Checking Rust code..."
	cd studio-core && cargo check

test-rust: ## Run Rust unit and integration tests
	@echo "🧪 Running Rust tests..."
	cd studio-core && cargo test

lint-rust: ## Lint Rust code using Clippy (treat warnings as errors)
	@echo "🧹 Linting Rust code..."
	cd studio-core && cargo clippy --all-targets --all-features -- -D warnings

fmt-rust: ## Format Rust code using rustfmt
	@echo "🎨 Formatting Rust code..."
	cd studio-core && cargo fmt

# ------------------------------------------------------------------------------
# 🐍 Python E2E Tests Commands
# ------------------------------------------------------------------------------
test-python: ## Run Python E2E tests using uv and pytest
	@echo "🐍 Running Python E2E tests..."
	uv run --with requests --with pytest -m pytest tests/e2e/ -v

lint-python: ## Lint Python tests using flake8
	@echo "🧹 Linting Python code..."
	uv run --with flake8 -m flake8 tests/ --max-line-length=120 --extend-ignore=E203,W503

fmt-python: ## Format Python tests using black
	@echo "🎨 Formatting Python code..."
	uv run --with black -m black tests/

# ------------------------------------------------------------------------------
# 🔄 Aggregate Commands
# ------------------------------------------------------------------------------
test: test-rust test-python ## Run all tests (Rust + Python E2E)

lint: lint-rust lint-python ## Run all linters (Clippy + Flake8)

fmt: fmt-rust fmt-python ## Format all code (rustfmt + black)

# ------------------------------------------------------------------------------
# 🐳 Docker Commands
# ------------------------------------------------------------------------------
docker-build: ## Build Docker images for the engine
	@echo "🐳 Building Docker images..."
	docker build -f docker/Dockerfile.engine -t studio-core:latest .

docker-up: ## Run the engine in a Docker container (background)
	@echo "🐳 Starting Docker container on port 3000..."
	docker run -d -p 3000:3000 --name doc-engine -e DOCUMENT_ENGINE_API_KEY=dev-secret studio-core:latest

docker-down: ## Stop and remove the Docker container
	@echo "🐳 Stopping Docker container..."
	docker stop doc-engine || true
	docker rm doc-engine || true

docker-logs: ## View live logs from the Docker container
	docker logs -f doc-engine

# ------------------------------------------------------------------------------
# 🧹 Utilities & Maintenance
# ------------------------------------------------------------------------------
tree: ## Display the project structure (ignoring build artifacts)
	@echo "🌳 Project Structure:"
	@tree -I 'target|.git|.zed|__pycache__|.pytest_cache|*.pyc' || echo "Install 'tree' to use this command."

clean: ## Clean all build artifacts, caches, and generated PDFs
	@echo "🧹 Cleaning build artifacts..."
	cd studio-core && cargo clean
	find . -type d -name "__pycache__" -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.pdf" -not -path "./docs/examples/*" -delete
	find . -type d -name ".pytest_cache" -exec rm -rf {} +
	@echo "✅ Clean complete."

seed: ## Seed the database with sample data (if applicable)
	@echo "🌱 Seeding data..."
	@uv run --with requests python scripts/seed_data.py || echo "⚠️  Seed script not found or failed."
