# aegnt-27 Proprietary Binary Distribution Makefile

# Configuration
PACKAGE_NAME = aegnt-27
VERSION = $(shell grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DIR = dist
RELEASE_DIR = releases

# Colors
GREEN = \033[0;32m
YELLOW = \033[1;33m
NC = \033[0m

.PHONY: all clean build-lite build-pro build-enterprise build-all package test-binaries install help

# Default target
all: build-all package

help:
	@echo "$(GREEN)aegnt-27 Proprietary Binary Distribution$(NC)"
	@echo ""
	@echo "Available targets:"
	@echo "  build-lite        - Build only aegnt-27-lite binary"
	@echo "  build-pro         - Build only aegnt-27-pro binary"
	@echo "  build-enterprise  - Build only aegnt-27-enterprise binary"
	@echo "  build-all         - Build all binary variants"
	@echo "  package          - Create distribution packages"
	@echo "  test-binaries    - Test compiled binaries"
	@echo "  clean            - Clean build artifacts"
	@echo "  install          - Install binaries locally"
	@echo "  release          - Complete build and package process"
	@echo ""
	@echo "Version: $(VERSION)"

# Build individual variants
build-lite:
	@echo "$(YELLOW)Building aegnt-27-lite...$(NC)"
	cargo build --release --bin aegnt-27-lite --features basic-humanization

build-pro:
	@echo "$(YELLOW)Building aegnt-27-pro...$(NC)"
	cargo build --release --bin aegnt-27-pro --features "all-humanization,detection-validation"

build-enterprise:
	@echo "$(YELLOW)Building aegnt-27-enterprise...$(NC)"
	cargo build --release --bin aegnt-27-enterprise --features full

# Build all variants
build-all: build-lite build-pro build-enterprise
	@echo "$(GREEN)All binaries built successfully!$(NC)"

# Cross-platform build
build-cross:
	@echo "$(YELLOW)Building for all supported platforms...$(NC)"
	./scripts/build-binaries.sh

# Create distribution packages
package: build-cross
	@echo "$(YELLOW)Creating distribution packages...$(NC)"
	./scripts/package-release.sh

# Test binaries
test-binaries: build-all
	@echo "$(YELLOW)Testing compiled binaries...$(NC)"
	@echo "Testing aegnt-27-lite..."
	./target/release/aegnt-27-lite info
	@echo ""
	@echo "Testing aegnt-27-pro..."
	./target/release/aegnt-27-pro info
	@echo ""
	@echo "Testing aegnt-27-enterprise..."
	./target/release/aegnt-27-enterprise info
	@echo ""
	@echo "$(GREEN)All binaries working correctly!$(NC)"

# Install binaries locally
install: build-all
	@echo "$(YELLOW)Installing binaries locally...$(NC)"
	cp target/release/aegnt-27-lite ~/.local/bin/ 2>/dev/null || sudo cp target/release/aegnt-27-lite /usr/local/bin/
	cp target/release/aegnt-27-pro ~/.local/bin/ 2>/dev/null || sudo cp target/release/aegnt-27-pro /usr/local/bin/
	cp target/release/aegnt-27-enterprise ~/.local/bin/ 2>/dev/null || sudo cp target/release/aegnt-27-enterprise /usr/local/bin/
	@echo "$(GREEN)Binaries installed successfully!$(NC)"

# Clean build artifacts
clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean
	rm -rf $(BUILD_DIR)
	rm -rf $(RELEASE_DIR)
	rm -f aegnt-27-v*-complete-release.tar.gz
	@echo "$(GREEN)Clean complete!$(NC)"

# Complete release process
release: clean build-cross package
	@echo "$(GREEN)Release $(VERSION) created successfully!$(NC)"
	@echo "Release archive: aegnt-27-v$(VERSION)-complete-release.tar.gz"

# Development helpers
dev-test: build-all
	@echo "$(YELLOW)Running development tests...$(NC)"
	./target/release/aegnt-27-lite mouse -x 0 -y 0 --end-x 100 --end-y 100
	./target/release/aegnt-27-pro typing -t "Test message" --wpm 60
	@echo "$(GREEN)Development tests completed!$(NC)"

# Check dependencies
check-deps:
	@echo "$(YELLOW)Checking build dependencies...$(NC)"
	@command -v rustup >/dev/null 2>&1 || { echo "rustup is required but not installed"; exit 1; }
	@command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed"; exit 1; }
	@echo "$(GREEN)Dependencies satisfied!$(NC)"

# Show build info
info:
	@echo "$(GREEN)aegnt-27 Build Information$(NC)"
	@echo "Package: $(PACKAGE_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Build directory: $(BUILD_DIR)"
	@echo "Release directory: $(RELEASE_DIR)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"