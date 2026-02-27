#!/bin/bash
# ZeroClaw Hackerbot Overlay - Build Script
# Builds the overlay with ZeroClaw as a dependency

set -e

echo "========================================"
echo "ZeroClaw Hackerbot Overlay Build"
echo "========================================"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo not found"
    echo "Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! command -v git &> /dev/null; then
    echo "ERROR: Git not found"
    exit 1
fi

echo "✓ Rust/Cargo found: $(cargo --version)"
echo "✓ Git found: $(git --version)"
echo ""

# Build type
BUILD_TYPE="${1:-release}"

echo "Build type: $BUILD_TYPE"
echo ""

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean
echo ""

# Update dependencies
echo "Updating ZeroClaw dependency..."
cargo update zeroclaw
echo ""

# Build
echo "Building..."
if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release
    BINARY_PATH="target/release/zeroclaw-hackerbot"
else
    cargo build
    BINARY_PATH="target/debug/zeroclaw-hackerbot"
fi
echo ""

# Test
echo "Running tests..."
cargo test --lib
echo ""

# Show binary info
echo "Binary information:"
ls -lh "$BINARY_PATH"
file "$BINARY_PATH"
echo ""

# Success
echo "========================================"
echo "Build successful!"
echo "========================================"
echo ""
echo "Binary: $BINARY_PATH"
echo ""
echo "Next steps:"
echo "1. Copy config: cp config/hackerbot-default.toml ~/.zeroclaw/hackerbot.toml"
echo "2. Edit config: nano ~/.zeroclaw/hackerbot.toml"
echo "3. Run: $BINARY_PATH --config ~/.zeroclaw/hackerbot.toml"
echo ""
