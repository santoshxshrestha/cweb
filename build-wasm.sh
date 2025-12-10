#!/bin/bash

# Build script to compile WASM and copy to public directory

set -e

echo "Building C Compiler WASM module..."

# Navigate to wasm directory and build
cd "$(dirname "$0")/wasm"
wasm-pack build --target web --release

# Copy the generated files to public directory
echo "Copying WASM files to public directory..."
mkdir -p ../public/wasm
cp pkg/c_compiler_wasm_bg.wasm ../public/wasm/
cp pkg/c_compiler_wasm.js ../public/wasm/

echo "✓ Build complete!"
echo "WASM files copied to public/wasm/"
echo ""
echo "You can now run: npm run dev"
