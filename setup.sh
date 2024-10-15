#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Step 1: Navigate to the sonobe_wasm_browser folder
cd "./sonobe_wasm_browser"

# Step 2: Build the wasm package for web target
wasm-pack build --target web

# Step 3: Move the generated files to the desired locations
# Move the .wasm file to the project root, then to public folder
mv ./pkg/sonobe_wasm_browser_bg.wasm ./../public/

# Move the .js file to the project root, then to public/js folder
mv ./pkg/sonobe_wasm_browser.js ./../public/js/

# Step 4: Clean up the pkg folder from the project root and sonobe_wasm folder
rm -rf pkg  # Remove pkg folder in the sonobe_wasm folder

echo "Build and file movements completed successfully!"
