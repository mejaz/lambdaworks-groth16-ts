// test.js
import * as wasm from './pkg/lambdaworks_groth16_wasm.js';

async function runTest() {
    try {
        console.log("Starting test...");

        // Initialize the WebAssembly module
        wasm.start();
        console.log("✅ WebAssembly module initialized successfully");

        // Create a test circuit setup
        const setup = new wasm.Groth16SetupWrapper({
            constraint_count: 10,
            variable_count: 5,
            public_input_count: 2
        });
        console.log("✅ Groth16Setup created successfully");

        // Generate keys
        const keyPair = setup.generate_keys();
        console.log("✅ Keys generated successfully:", keyPair);

        // Create a prover
        const prover = new wasm.Groth16ProverWrapper(keyPair.proving_key);
        console.log("✅ Groth16Prover created successfully");

        // Create a test proof
        const publicInputs = ["0x01", "0x02"];
        const privateInputs = ["0x03", "0x04", "0x05"];
        const proof = prover.create_proof(publicInputs, privateInputs);
        console.log("✅ Proof created successfully:", proof);

        // Verify the proof
        const isValid = wasm.verify_proof(keyPair.verifying_key, proof, publicInputs);
        console.log("✅ Proof verification result:", isValid);

        console.log("🎉 All tests passed successfully!");
    } catch (error) {
        console.error("❌ Test failed:", error);
        console.error(error.stack);
    }
}

runTest();