#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Compile the circuit and get `r1cs` and `wasm` outputs.
circom ./circuits/with_external_inputs.circom --r1cs --wasm --prime bn128 --output ./

# Move `r1cs` and `wasm` outpus to `./public` to be served with our site statically.
mv ./with_external_inputs_js/with_external_inputs.wasm ./public
mv with_external_inputs.r1cs ./public

# Remove the folder.
rm -rf ./with_external_inputs_js

