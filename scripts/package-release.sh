#!/bin/bash
# Release packaging script for aegnt-27 proprietary binaries

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== aegnt-27 Release Packaging Script ===${NC}"
echo

# Configuration
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
RELEASE_DIR="releases"
DIST_DIR="dist"

echo -e "${YELLOW}Packaging aegnt-27 v${VERSION} for release${NC}"
echo

# Check if dist directory exists
if [ ! -d "${DIST_DIR}" ]; then
    echo -e "${RED}Error: ${DIST_DIR} directory not found. Run build-binaries.sh first.${NC}"
    exit 1
fi

# Create release directory
echo -e "${YELLOW}Creating release directory...${NC}"
rm -rf ${RELEASE_DIR}
mkdir -p ${RELEASE_DIR}

# Function to create release manifest
create_manifest() {
    local manifest_file="${RELEASE_DIR}/release-manifest.json"
    
    echo -e "${GREEN}Creating release manifest...${NC}"
    
    cat > ${manifest_file} << EOF
{
  "product": "aegnt-27",
  "version": "${VERSION}",
  "release_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "variants": {
    "lite": {
      "description": "Basic humanization features",
      "features": ["mouse-humanization", "typing-humanization"],
      "target_users": "Individual developers, basic automation"
    },
    "pro": {
      "description": "Full feature set with AI detection",
      "features": ["mouse-humanization", "typing-humanization", "audio-humanization", "visual-humanization", "ai-detection"],
      "target_users": "Professional developers, content creators"
    },
    "enterprise": {
      "description": "Advanced ML models and enterprise features",
      "features": ["all-features", "api-server", "ml-training", "analytics", "management-tools"],
      "target_users": "Enterprise customers, large teams"
    }
  },
  "platforms": [
    {
      "target": "x86_64-unknown-linux-gnu",
      "os": "Linux",
      "arch": "x86_64",
      "format": "tar.gz"
    },
    {
      "target": "x86_64-pc-windows-gnu",
      "os": "Windows",
      "arch": "x86_64",
      "format": "zip"
    },
    {
      "target": "x86_64-apple-darwin",
      "os": "macOS",
      "arch": "x86_64",
      "format": "tar.gz"
    }
  ],
  "installation_notes": {
    "linux": "Extract and add to PATH. May require libc6-dev for some features.",
    "windows": "Extract to desired location. Windows Defender may flag binaries initially.",
    "macos": "Extract and run. May need to allow in Security & Privacy settings."
  },
  "licensing": {
    "lite": "Commercial License Required",
    "pro": "Commercial License Required",
    "enterprise": "Enterprise License Required"
  }
}
EOF
    
    echo -e "  ✓ Created ${manifest_file}"
}

# Function to create installation scripts
create_install_scripts() {
    echo -e "${GREEN}Creating installation scripts...${NC}"
    
    # Linux/macOS installation script
    cat > ${RELEASE_DIR}/install.sh << 'EOF'
#!/bin/bash
# aegnt-27 Installation Script

set -e

# Configuration
INSTALL_DIR="/usr/local/bin"
TEMP_DIR=$(mktemp -d)

echo "=== aegnt-27 Installation ==="
echo

# Check if running as root for system-wide install
if [ "$EUID" -eq 0 ]; then
    echo "Installing system-wide to ${INSTALL_DIR}..."
else
    INSTALL_DIR="$HOME/.local/bin"
    echo "Installing for current user to ${INSTALL_DIR}..."
    mkdir -p ${INSTALL_DIR}
fi

# Detect platform
PLATFORM=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="x86_64-unknown-linux-gnu"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="x86_64-apple-darwin"
else
    echo "Error: Unsupported platform: $OSTYPE"
    exit 1
fi

# Check if archive exists
ARCHIVE="aegnt-27-v*-${PLATFORM}.tar.gz"
if ! ls ${ARCHIVE} 1> /dev/null 2>&1; then
    echo "Error: No installation archive found for ${PLATFORM}"
    echo "Expected: ${ARCHIVE}"
    exit 1
fi

# Extract and install
echo "Extracting archive..."
tar -xzf ${ARCHIVE} -C ${TEMP_DIR}
EXTRACTED_DIR=$(find ${TEMP_DIR} -name "aegnt-27-v*-${PLATFORM}" -type d)

echo "Installing binaries to ${INSTALL_DIR}..."
cp ${EXTRACTED_DIR}/aegnt-27-* ${INSTALL_DIR}/
chmod +x ${INSTALL_DIR}/aegnt-27-*

# Cleanup
rm -rf ${TEMP_DIR}

echo
echo "Installation completed successfully!"
echo
echo "Available commands:"
echo "  aegnt-27-lite       - Basic humanization features"
echo "  aegnt-27-pro        - Full feature set"
echo "  aegnt-27-enterprise - Enterprise features"
echo
echo "Run any command with --help for usage information."

# Add to PATH reminder
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]] && [ "$EUID" -ne 0 ]; then
    echo
    echo "NOTE: Add $HOME/.local/bin to your PATH:"
    echo "  echo 'export PATH=\$PATH:$HOME/.local/bin' >> ~/.bashrc"
    echo "  source ~/.bashrc"
fi
EOF
    
    chmod +x ${RELEASE_DIR}/install.sh
    
    # Windows installation script
    cat > ${RELEASE_DIR}/install.bat << 'EOF'
@echo off
REM aegnt-27 Installation Script for Windows

echo === aegnt-27 Installation ===
echo.

REM Check for admin privileges
net session >nul 2>&1
if %errorLevel% == 0 (
    set "INSTALL_DIR=C:\Program Files\aegnt-27"
    echo Installing system-wide to %INSTALL_DIR%...
) else (
    set "INSTALL_DIR=%USERPROFILE%\aegnt-27"
    echo Installing for current user to %INSTALL_DIR%...
)

REM Create installation directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Check if archive exists
if not exist "aegnt-27-v*-x86_64-pc-windows-gnu.zip" (
    echo Error: No installation archive found for Windows
    pause
    exit /b 1
)

REM Extract archive (requires PowerShell)
echo Extracting archive...
powershell -command "Expand-Archive -Path 'aegnt-27-v*-x86_64-pc-windows-gnu.zip' -DestinationPath '%TEMP%' -Force"

REM Find extracted directory
for /d %%i in ("%TEMP%\aegnt-27-v*-x86_64-pc-windows-gnu") do set "EXTRACTED_DIR=%%i"

REM Copy binaries
echo Installing binaries to %INSTALL_DIR%...
copy "%EXTRACTED_DIR%\*.exe" "%INSTALL_DIR%\"

REM Add to PATH (user-level)
if %errorLevel% neq 0 (
    echo Adding to PATH...
    setx PATH "%PATH%;%INSTALL_DIR%"
)

echo.
echo Installation completed successfully!
echo.
echo Available commands:
echo   aegnt-27-lite.exe       - Basic humanization features
echo   aegnt-27-pro.exe        - Full feature set
echo   aegnt-27-enterprise.exe - Enterprise features
echo.
echo Run any command with --help for usage information.
echo.
pause
EOF
    
    echo -e "  ✓ Created installation scripts"
}

# Function to create documentation
create_docs() {
    echo -e "${GREEN}Creating release documentation...${NC}"
    
    cat > ${RELEASE_DIR}/QUICK-START.md << EOF
# aegnt-27 Quick Start Guide

## Overview

aegnt-27 is a proprietary humanization system that provides three tiers of functionality:

- **aegnt-27-lite**: Basic mouse and typing humanization
- **aegnt-27-pro**: Full feature set with AI detection
- **aegnt-27-enterprise**: Advanced ML models and enterprise features

## Installation

### Linux/macOS
\`\`\`bash
./install.sh
\`\`\`

### Windows
Run \`install.bat\` as administrator or double-click.

## Basic Usage

### aegnt-27-lite
\`\`\`bash
# Humanize mouse movement
aegnt-27-lite mouse -x 100 -y 100 --end-x 500 --end-y 300

# Humanize typing
aegnt-27-lite typing -t "Hello, world!" --wpm 65
\`\`\`

### aegnt-27-pro
\`\`\`bash
# Advanced mouse with curves
aegnt-27-pro mouse -x 100 -y 100 --end-x 500 --end-y 300 --curve 0.3

# Audio synthesis
aegnt-27-pro audio -t "Hello, world!" --voice neutral -o output.wav

# AI detection
aegnt-27-pro detect -i content.txt --model ensemble
\`\`\`

### aegnt-27-enterprise
\`\`\`bash
# Start API server
aegnt-27-enterprise server -p 8080

# Train custom models
aegnt-27-enterprise ml-train -m mouse -d training_data/ -o model.bin

# Performance analytics
aegnt-27-enterprise analytics -i logs/ -t performance
\`\`\`

## Getting Help

Run any command with \`--help\` for detailed usage information:
\`\`\`bash
aegnt-27-lite --help
aegnt-27-pro typing --help
aegnt-27-enterprise server --help
\`\`\`

## License

This is proprietary software. See LICENSE file for terms.
EOF
    
    echo -e "  ✓ Created quick start guide"
}

# Function to create verification script
create_verification() {
    echo -e "${GREEN}Creating verification script...${NC}"
    
    cat > ${RELEASE_DIR}/verify-installation.sh << 'EOF'
#!/bin/bash
# aegnt-27 Installation Verification Script

echo "=== aegnt-27 Installation Verification ==="
echo

# Check if binaries are in PATH
BINARIES=("aegnt-27-lite" "aegnt-27-pro" "aegnt-27-enterprise")
ALL_FOUND=true

for binary in "${BINARIES[@]}"; do
    if command -v ${binary} &> /dev/null; then
        echo "✓ ${binary} found in PATH"
        ${binary} info &> /dev/null && echo "  - Binary responds correctly"
    else
        echo "✗ ${binary} not found in PATH"
        ALL_FOUND=false
    fi
done

echo

if [ "$ALL_FOUND" = true ]; then
    echo "✓ All aegnt-27 binaries installed successfully!"
    echo
    echo "Test commands:"
    echo "  aegnt-27-lite info"
    echo "  aegnt-27-pro info"
    echo "  aegnt-27-enterprise info"
else
    echo "✗ Some binaries are missing. Please run the installation script."
fi
EOF
    
    chmod +x ${RELEASE_DIR}/verify-installation.sh
    
    echo -e "  ✓ Created verification script"
}

# Copy distribution files
echo -e "${GREEN}Copying distribution files...${NC}"
cp -r ${DIST_DIR}/*.tar.gz ${DIST_DIR}/*.zip ${RELEASE_DIR}/ 2>/dev/null || true
cp ${DIST_DIR}/checksums.sha256 ${RELEASE_DIR}/

# Create release components
create_manifest
create_install_scripts
create_docs
create_verification

# Create release notes
echo -e "${GREEN}Creating release notes...${NC}"
cat > ${RELEASE_DIR}/RELEASE-NOTES.md << EOF
# aegnt-27 v${VERSION} Release Notes

Release Date: $(date +%Y-%m-%d)

## What's Included

This release includes three binary variants:

### aegnt-27-lite
- Basic mouse and typing humanization
- JSON export capabilities
- Cross-platform support
- Minimal dependencies

### aegnt-27-pro
- All lite features plus:
- Advanced mouse curves and jitter
- Audio synthesis with voice types
- Visual pattern analysis
- AI detection validation
- Batch processing

### aegnt-27-enterprise
- All pro features plus:
- High-performance API server
- Custom ML model training
- Advanced processing pipelines
- Performance analytics
- Enterprise management tools
- Audit logging

## Platform Support

- Linux (x86_64)
- Windows (x86_64)
- macOS (x86_64)

## Installation

See QUICK-START.md for installation instructions.

## Verification

Run \`verify-installation.sh\` to verify your installation.

## License

This is proprietary software requiring a valid license key.
Contact sales for licensing information.
EOF

# Create final release archive
echo -e "${GREEN}Creating final release archive...${NC}"
RELEASE_ARCHIVE="aegnt-27-v${VERSION}-complete-release.tar.gz"
(cd releases && tar -czf "../${RELEASE_ARCHIVE}" .)

# Final summary
echo -e "\n${GREEN}=== Release Packaging Complete ===${NC}"
echo -e "Version: ${VERSION}"
echo -e "Release directory: ${RELEASE_DIR}"
echo -e "Complete archive: ${RELEASE_ARCHIVE}"
echo
echo -e "${BLUE}Release contents:${NC}"
ls -la ${RELEASE_DIR}/
echo
echo -e "${GREEN}Release packaging completed successfully!${NC}"