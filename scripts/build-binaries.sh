#!/bin/bash
# Build script for aegnt-27 proprietary binary distribution

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== aegnt-27 Binary Distribution Build Script ===${NC}"
echo

# Configuration
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DIR="dist"
TARGETS=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "x86_64-apple-darwin")

echo -e "${YELLOW}Building aegnt-27 v${VERSION}${NC}"
echo

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
rm -rf ${BUILD_DIR}
mkdir -p ${BUILD_DIR}

# Function to build a binary for a target
build_binary() {
    local binary_name=$1
    local target=$2
    local features=$3
    
    echo -e "${GREEN}Building ${binary_name} for ${target}...${NC}"
    
    # Determine file extension
    local ext=""
    if [[ $target == *"windows"* ]]; then
        ext=".exe"
    fi
    
    # Build the binary
    cargo build --release --bin ${binary_name} --features "${features}" --target ${target}
    
    # Copy to dist directory
    local target_dir="${BUILD_DIR}/${target}"
    mkdir -p ${target_dir}
    cp "target/${target}/release/${binary_name}${ext}" "${target_dir}/"
    
    echo -e "  ✓ Built ${target_dir}/${binary_name}${ext}"
}

# Function to create distribution package
create_package() {
    local target=$1
    local target_dir="${BUILD_DIR}/${target}"
    
    echo -e "${GREEN}Creating distribution package for ${target}...${NC}"
    
    # Create package directory
    local package_dir="${BUILD_DIR}/aegnt-27-v${VERSION}-${target}"
    mkdir -p ${package_dir}
    
    # Copy binaries
    cp ${target_dir}/* ${package_dir}/
    
    # Copy documentation
    cp README.md ${package_dir}/
    cp LICENSE ${package_dir}/
    cp CHANGELOG.md ${package_dir}/
    
    # Create package info
    cat > ${package_dir}/PACKAGE-INFO.txt << EOF
aegnt-27 Proprietary Binary Distribution
Version: ${VERSION}
Target: ${target}
Build Date: $(date)

Binaries included:
- aegnt-27-lite: Basic humanization features (mouse, typing)
- aegnt-27-pro: Full feature set with AI detection
- aegnt-27-enterprise: Advanced ML models and enterprise features

For usage information, run each binary with --help
EOF
    
    # Create archive
    if [[ $target == *"windows"* ]]; then
        (cd ${BUILD_DIR} && zip -r "aegnt-27-v${VERSION}-${target}.zip" "aegnt-27-v${VERSION}-${target}/")
    else
        (cd ${BUILD_DIR} && tar -czf "aegnt-27-v${VERSION}-${target}.tar.gz" "aegnt-27-v${VERSION}-${target}/")
    fi
    
    echo -e "  ✓ Created distribution package"
}

# Check if required targets are installed
echo -e "${YELLOW}Checking target availability...${NC}"
for target in "${TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "^${target}$"; then
        echo -e "${YELLOW}Installing target ${target}...${NC}"
        rustup target add ${target}
    fi
done

# Build all binaries for all targets
for target in "${TARGETS[@]}"; do
    echo -e "\n${GREEN}=== Building for ${target} ===${NC}"
    
    # Build lite version
    build_binary "aegnt-27-lite" ${target} "basic-humanization"
    
    # Build pro version
    build_binary "aegnt-27-pro" ${target} "all-humanization,detection-validation"
    
    # Build enterprise version (with full features)
    build_binary "aegnt-27-enterprise" ${target} "full"
    
    # Create distribution package
    create_package ${target}
done

# Create checksums
echo -e "\n${GREEN}Creating checksums...${NC}"
(cd ${BUILD_DIR} && find . -name "*.tar.gz" -o -name "*.zip" | xargs sha256sum > checksums.sha256)

# Build summary
echo -e "\n${GREEN}=== Build Summary ===${NC}"
echo -e "Version: ${VERSION}"
echo -e "Build directory: ${BUILD_DIR}"
echo -e "Targets built: ${#TARGETS[@]}"
echo -e "Binaries per target: 3 (lite, pro, enterprise)"
echo
echo -e "${GREEN}Distribution packages:${NC}"
ls -la ${BUILD_DIR}/*.tar.gz ${BUILD_DIR}/*.zip 2>/dev/null || true
echo
echo -e "${GREEN}Build completed successfully!${NC}"