# Makefile for Fesch Agent
# Build, run, and manage the polyglot agent system

.PHONY: all build clean run test native core brain adk-web help

# Default target
all: build

# Build everything
build: native core brain
	@echo "✅ Build complete"

# Build C++ native layer
native:
	@echo "🔨 Building C++ native layer..."
	mkdir -p native/build
	cd native/build && cmake .. -DCMAKE_BUILD_TYPE=Release
	cd native/build && cmake --build .

# Build Rust core
core:
	@echo "🦀 Building Rust core..."
	cd core && cargo build --release

# Install Python brain
brain:
	@echo "🐍 Installing Python brain..."
	uv sync
	uv run playwright install

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	rm -rf native/build
	rm -rf core/target
	rm -rf brain/__pycache__
	rm -rf brain/tools/__pycache__
	rm -rf brain/skills/__pycache__
	rm -rf brain/plugins/__pycache__
	rm -f *.pyc
	@echo "✅ Clean complete"

# Run the TUI
run:
	@echo "🚀 Running Fesch Agent TUI..."
	cd core && cargo run --release

# Run in dev mode (without TUI, CLI only)
run-cli:
	@echo "🚀 Running Fesch Agent CLI..."
	uv run adk run brain/ --message "Hello"

# Run ADK web UI
adk-web:
	@echo "🌐 Starting ADK web UI..."
	uv run adk web brain/

# Run tests
test:
	@echo "🧪 Running tests..."
	uv run pytest

# Install dependencies
install:
	@echo "📦 Installing dependencies..."
	uv sync
	cd core && cargo build

# Format code
format:
	@echo "✨ Formatting code..."
	cd core && cargo fmt
	uv run ruff format brain/

# Lint code
lint:
	@echo "🔍 Linting code..."
	cd core && cargo clippy -- -D warnings
	uv run ruff check brain/

# Help
help:
	@echo "Fesch Agent - Polyglot AI Agent System"
	@echo ""
	@echo "Usage:"
	@echo "  make build     - Build everything"
	@echo "  make native    - Build C++ native layer"
	@echo "  make core      - Build Rust core"
	@echo "  make brain     - Install Python brain"
	@echo "  make run       - Run the TUI"
	@echo "  make run-cli   - Run CLI mode"
	@echo "  make adk-web   - Start ADK web UI"
	@echo "  make clean     - Clean build artifacts"
	@echo "  make test      - Run tests"
	@echo "  make install   - Install dependencies"
	@echo "  make format    - Format code"
	@echo "  make lint      - Lint code"
	@echo "  make help      - Show this help"
