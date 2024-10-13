// Import the init and single_fold functions from the generated JS file
import init, { single_fold } from './sonobe_wasm_browser.js';  // Update to your actual path

export async function loadAndCallSingleFold(r1cs_bytes, witness_js) {
    try {
        // Step 1: Initialize the WASM module
        await init("sonobe_wasm_browser_bg.wasm");  // This loads and initializes the WASM module

        try {
            // Step 2: Call the single_fold function with the Uint8Array inputs
            let res = single_fold(r1cs_bytes, witness_js); // Call the Rust/WASM function
        } catch (e) {
            console.error("Caught Rust panic in JS:", e);
        }


        console.log('single_fold executed successfully');
        return res;
    } catch (error) {
        console.error('Error loading or executing the WebAssembly module:', error);
    }
}

