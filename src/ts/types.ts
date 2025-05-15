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