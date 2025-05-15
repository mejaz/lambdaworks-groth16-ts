import {
  start,
  Groth16SetupWrapper,
  Groth16ProverWrapper,
  verify_proof
} from '../../pkg';

/**
 * Initializes the WebAssembly module
 * @returns A promise that resolves when WebAssembly is initialized
 */
export async function initialize(): Promise<void> {
  // Call the start function exported by the WebAssembly module
  start();
}

/**
 * Class for setting up a Groth16 proving system
 */
export class Groth16Setup {
  private wasmSetup: Groth16SetupWrapper;

  /**
   * Create a new Groth16 setup
   * @param params Circuit parameters
   */
  constructor(params: CircuitParams) {
    this.wasmSetup = new Groth16SetupWrapper({
      constraint_count: params.constraintCount,
      variable_count: params.variableCount,
      public_input_count: params.publicInputCount
    });
  }

  /**
   * Generate proving and verifying keys
   * @returns A key pair containing proving and verifying keys
   */
  generateKeys(): KeyPair {
    return this.wasmSetup.generate_keys() as KeyPair;
  }
}

/**
 * Class for generating proofs
 */
export class Groth16Prover {
  private wasmProver: Groth16ProverWrapper;

  /**
   * Create a new prover
   * @param provingKey The proving key
   */
  constructor(provingKey: SerializedProvingKey) {
    this.wasmProver = new Groth16ProverWrapper(provingKey);
  }

  /**
   * Create a proof
   * @param publicInputs Public inputs as hex strings
   * @param privateInputs Private inputs as hex strings
   * @returns The generated proof
   */
  createProof(publicInputs: string[], privateInputs: string[]): SerializedProof {
    return this.wasmProver.create_proof(publicInputs, privateInputs) as SerializedProof;
  }
}

/**
 * Verify a proof
 * @param verifyingKey The verifying key
 * @param proof The proof to verify
 * @param publicInputs Public inputs as hex strings
 * @returns True if the proof is valid, false otherwise
 */
export function verifyProof(
  verifyingKey: SerializedVerifyingKey,
  proof: SerializedProof,
  publicInputs: string[]
): boolean {
  return verify_proof(verifyingKey, proof, publicInputs);
}

// Re-export types
export interface CircuitParams {
  constraintCount: number;
  variableCount: number;
  publicInputCount: number;
}

export interface KeyPair {
  provingKey: SerializedProvingKey;
  verifyingKey: SerializedVerifyingKey;
}

export interface SerializedProvingKey {
  data: string[];
}

export interface SerializedVerifyingKey {
  data: string[];
}

export interface SerializedProof {
  a: string;
  b: string;
  c: string;
}