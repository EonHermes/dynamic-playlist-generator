.PHONY: build test run clean dev-frontend dev-backend release install

# Build variables
BACKEND_DIR = backend
FRONTEND_DIR = frontend
RELEASE_DIR = target/release
BINARY_NAME = dynamic-playlist-generator

# Default target
all: build

# Build everything
build: build-backend build-frontend

# Build backend
build-backend:
	@echo "Building Rust backend..."
	cd $(BACKEND_DIR) && cargo build --release
	@echo "Backend built: $(BACKEND_DIR)/$(RELEASE_DIR)/$(BINARY_NAME)"

# Build frontend
build-frontend:
	@echo "Building React frontend..."
	cd $(FRONTEND_DIR) && npm install && npm run build
	@echo "Frontend built: $(FRONTEND_DIR)/dist/"

# Run backend server
run:
	@echo "Starting backend server..."
	cd $(BACKEND_DIR) && cargo run --release

# Run frontend dev server
dev-frontend:
	@echo "Starting frontend dev server..."
	cd $(FRONTEND_DIR) && npm run dev

# Run both in development (requires tmux or separate terminals)
dev: build-backend
	@echo "Frontend dev: cd $(FRONTEND_DIR) && npm run dev"
	@echo "Backend dev: cd $(BACKEND_DIR) && cargo run"

# Run tests
test:
	@echo "Running tests..."
	cd $(BACKEND_DIR) && cargo test --release
	cd $(FRONTEND_DIR) && npm test 2>/dev/null || echo "No frontend tests configured"

# Clean build artifacts
clean:
	@echo "Cleaning..."
	cd $(BACKEND_DIR) && cargo clean
	cd $(FRONTEND_DIR) && rm -rf dist node_modules/.vite
	rm -rf $(FRONTEND_DIR)/node_modules

# Create release package
release: build
	@mkdir -p release
	cp $(BACKEND_DIR)/$(RELEASE_DIR)/$(BINARY_NAME) release/
	cp -r $(FRONTEND_DIR)/dist release/frontend
	cp README.md LICENSE release/
	tar -czf dynamic-playlist-generator-release.tar.gz release/
	@echo "Release package: dynamic-playlist-generator-release.tar.gz"

# Docker build (if Dockerfile exists)
docker:
	docker build -t dynamic-playlist-generator:latest .

# Install system dependencies check
check-deps:
	@echo "Checking dependencies..."
	@command -v cargo >/dev/null 2>&1 || echo "Rust not installed: https://rustup.rs/"
	@command -v npm >/dev/null 2>&1 || echo "Node.js/npm not installed"
	@echo "Dependencies OK"

# Quick start (build + run)
quickstart: build run
