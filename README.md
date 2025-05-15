````markdown
# lambdaworks-groth16-ts

TypeScript bindings for the Lambdaworks Groth16 zero-knowledge proof system.

## Installation

```bash
npm install lambdaworks-groth16-ts
````

## Usage

```typescript
import { initialize, Groth16Setup, Groth16Prover, verifyProof } from 'lambdaworks-groth16-ts';

async function example() {
  // Initialize WebAssembly module
  await initialize();

  // Set up a circuit
  const setup = new Groth16Setup({
    constraintCount: 10,
    variableCount: 5,
    publicInputCount: 2
  });

  // Generate proving and verifying keys
  const keyPair = setup.generateKeys();

  // Create a prover
  const prover = new Groth16Prover(keyPair.provingKey);

  // Create a proof
  const publicInputs = ["0x01", "0x02"];
  const privateInputs = ["0x03", "0x04", "0x05"];
  const proof = prover.createProof(publicInputs, privateInputs);

  // Verify the proof
  const isValid = verifyProof(keyPair.verifyingKey, proof, publicInputs);
  console.log(`Proof verification result: ${isValid}`);
}

example();
```

## API Reference

### `initialize()`

Initializes the WebAssembly module. This function must be called before using any other functions.

### `Groth16Setup`

Class for setting up a Groth16 proving system.

* **`constructor(params: CircuitParams)`**
  Creates a new setup with the given circuit parameters.
* **`generateKeys(): KeyPair`**
  Generates proving and verifying keys for the circuit.

### `Groth16Prover`

Class for generating proofs.

* **`constructor(provingKey: SerializedProvingKey)`**
  Creates a new prover with the given proving key.
* **`createProof(publicInputs: string[], privateInputs: string[]): SerializedProof`**
  Creates a proof with the given public and private inputs.
* `verifyProof(verifyingKey: SerializedVerifyingKey, proof: SerializedProof, publicInputs: string[]): boolean`


Verifies a proof with the given verifying key and public inputs.

## Types

### `CircuitParams`

```typescript
interface CircuitParams {
  constraintCount: number;
  variableCount: number;
  publicInputCount: number;
}
```

### `KeyPair`

```typescript
interface KeyPair {
  provingKey: SerializedProvingKey;
  verifyingKey: SerializedVerifyingKey;
}
```

### `SerializedProvingKey`

```typescript
interface SerializedProvingKey {
  data: string[];
}
```

### `SerializedVerifyingKey`

```typescript
interface SerializedVerifyingKey {
  data: string[];
}
```

### `SerializedProof`

```typescript
interface SerializedProof {
  a: string;
  b: string;
  c: string;
}
```

## Important Note

This package currently uses a simplified implementation for demonstration purposes. Future versions will integrate with the full Lambdaworks Groth16 implementation for production use.

## License

MIT
