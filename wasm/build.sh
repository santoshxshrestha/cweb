#!/bin/bash

# Build script for C Online Compiler WASM module

set -e

echo "Building C Compiler WASM module..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM module
echo "Running wasm-pack build..."
wasm-pack build --target web --release

echo ""
echo "Build complete! The WASM module is in the 'pkg' directory."
echo ""
echo "To test locally, run:"
echo "  python3 -m http.server 8000"
echo "  # or"
echo "  npx http-server"
echo ""
echo "Then open http://localhost:8000 in your browser."
