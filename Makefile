.PHONY: all desktop web clean install-web-target setup-web help dist-desktop dist-web

# Project name
PROJECT_NAME = rust_raylib
GAME_TITLE = Snake Game

# Build directories
BUILD_DIR = build
WEB_DIR = $(BUILD_DIR)/web
DESKTOP_DIR = $(BUILD_DIR)/desktop

# Default target
all: desktop

help:
	@echo "Rust Raylib Snake Game - Build Commands"
	@echo "========================================"
	@echo "make desktop       - Build for desktop (current platform)"
	@echo "make web           - Build for web (WASM)"
	@echo "make dist-desktop  - Build and package desktop release"
	@echo "make dist-web      - Build and package web release for itch.io"
	@echo "make all           - Build desktop (default)"
	@echo "make clean         - Clean build artifacts"
	@echo "make setup-web     - Install web build dependencies"
	@echo ""

# Desktop build
desktop:
	@echo "Building for desktop..."
	@cargo build --release
	@echo "Desktop build complete! Binary: target/release/$(PROJECT_NAME)"

# Package desktop build
dist-desktop: desktop
	@echo "Packaging desktop build..."
	@mkdir -p $(DESKTOP_DIR)
	@cp target/release/$(PROJECT_NAME) $(DESKTOP_DIR)/
	@echo "Desktop build packaged in $(DESKTOP_DIR)/"

# Setup web build dependencies
setup-web:
	@echo "Setting up web build dependencies..."
	@echo "Checking for emscripten target..."
	@rustup target add wasm32-unknown-emscripten || true
	@echo ""
	@echo "=========================================="
	@echo "IMPORTANT: You need to install Emscripten SDK"
	@echo "=========================================="
	@echo "Visit: https://emscripten.org/docs/getting_started/downloads.html"
	@echo ""
	@echo "Quick install (Linux/macOS):"
	@echo "  git clone https://github.com/emscripten-core/emsdk.git"
	@echo "  cd emsdk"
	@echo "  ./emsdk install latest"
	@echo "  ./emsdk activate latest"
	@echo "  source ./emsdk_env.sh"
	@echo ""
	@echo "After installing Emscripten, run: make web"
	@echo ""

# Check if emscripten is available
check-emscripten:
	@which emcc > /dev/null 2>&1 || (echo "Error: Emscripten not found. Run 'make setup-web' for instructions." && exit 1)

# Web build
web: check-emscripten
	@echo "Building for web (WASM)..."
	EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1" \
	cargo build --release --target wasm32-unknown-emscripten
	@echo "Web build complete!"

# Package web build for itch.io
dist-web: check-emscripten
	@echo "Building and packaging for web (itch.io)..."
	@mkdir -p $(WEB_DIR)
	@echo "Compiling to WASM..."
	EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1" \
	cargo build --release --target wasm32-unknown-emscripten
	@echo "Copying web files..."
	@cp target/wasm32-unknown-emscripten/release/$(PROJECT_NAME).js $(WEB_DIR)/
	@cp target/wasm32-unknown-emscripten/release/$(PROJECT_NAME).wasm $(WEB_DIR)/
	@echo "Generating index.html..."
	@sed 's/GAME_JS_FILE/$(PROJECT_NAME).js/g' index.html.template > $(WEB_DIR)/index.html
	@echo "Creating itch.io package..."
	@cd $(WEB_DIR) && zip -r ../$(PROJECT_NAME)-web.zip * > /dev/null
	@echo ""
	@echo "=========================================="
	@echo "Web build complete!"
	@echo "=========================================="
	@echo "Package: $(BUILD_DIR)/$(PROJECT_NAME)-web.zip"
	@echo ""
	@echo "To upload to itch.io:"
	@echo "1. Go to your game's page on itch.io"
	@echo "2. Upload $(PROJECT_NAME)-web.zip"
	@echo "3. Check 'This file will be played in the browser'"
	@echo "4. Set viewport dimensions to 800x600 (or your game's resolution)"
	@echo ""

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf $(BUILD_DIR)
	@echo "Clean complete!"
