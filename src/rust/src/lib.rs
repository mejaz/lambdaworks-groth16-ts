use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use lambdaworks_math::field::{
    element::FieldElement,
    fields::u64_prime_field::U64PrimeField,
};

// Initialize console error panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_exports_info() -> String {
    "Rust module exports: Groth16SetupWrapper, Groth16ProverWrapper, verify_proof".to_string()
}

// Define our field type - use a 61-bit prime field
type Fr = U64PrimeField<2305843009213693951>; // 2^61 - 1 (Mersenne prime)

#[wasm_bindgen]
pub struct Groth16SetupWrapper {
    constraint_count: usize,
    variable_count: usize,
    public_input_count: usize,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct CircuitParams {
    pub constraint_count: usize,
    pub variable_count: usize,
    pub public_input_count: usize,
}

#[wasm_bindgen]
impl Groth16SetupWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> Result<Groth16SetupWrapper, JsError> {
        let circuit_params: CircuitParams = serde_wasm_bindgen::from_value(params.clone())?;

        Ok(Groth16SetupWrapper {
            constraint_count: circuit_params.constraint_count,
            variable_count: circuit_params.variable_count,
            public_input_count: circuit_params.public_input_count
        })
    }

    #[wasm_bindgen]
    pub fn generate_keys(&self) -> Result<JsValue, JsError> {
        // For now, create dummy proving and verifying keys
        // In a real implementation, you would use lambdaworks-groth16 to generate these

        let proving_key = SerializedProvingKey {
            // Create identifiable dummy data
            data: vec![
                format!("pk_field_1_{}", self.constraint_count),
                format!("pk_field_2_{}", self.variable_count),
                format!("pk_field_3_{}", self.public_input_count),
            ],
        };

        let verifying_key = SerializedVerifyingKey {
            // Create identifiable dummy data
            data: vec![
                format!("vk_field_1_{}", self.constraint_count),
                format!("vk_field_2_{}", self.variable_count),
                format!("vk_field_3_{}", self.public_input_count),
            ],
        };

        let keys = KeyPair {
            proving_key,
            verifying_key,
        };

        Ok(serde_wasm_bindgen::to_value(&keys)?)
    }
}

#[wasm_bindgen]
pub struct Groth16ProverWrapper {
    proving_key: SerializedProvingKey,
}

#[wasm_bindgen]
impl Groth16ProverWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(proving_key_js: &JsValue) -> Result<Groth16ProverWrapper, JsError> {
        let proving_key: SerializedProvingKey = serde_wasm_bindgen::from_value(proving_key_js.clone())?;

        Ok(Groth16ProverWrapper {
            proving_key
        })
    }

    #[wasm_bindgen]
    pub fn create_proof(
        &self,
        public_inputs_js: &JsValue,
        private_inputs_js: &JsValue
    ) -> Result<JsValue, JsError> {
        let public_inputs: Vec<String> = serde_wasm_bindgen::from_value(public_inputs_js.clone())?;
        let private_inputs: Vec<String> = serde_wasm_bindgen::from_value(private_inputs_js.clone())?;

        // Convert hex strings to field elements (just for validation)
        let _public_inputs_fe = convert_hex_to_field_elements(&public_inputs)?;
        let _private_inputs_fe = convert_hex_to_field_elements(&private_inputs)?;

        // Create dummy proof for now
        // In a real implementation, you would generate an actual Groth16 proof
        let proof = SerializedProof {
            a: format!("proof_a_{}", public_inputs.first().unwrap_or(&"0".to_string())),
            b: format!("proof_b_{}", private_inputs.first().unwrap_or(&"0".to_string())),
            c: format!("proof_c_{}", self.proving_key.data.first().unwrap_or(&"0".to_string())),
        };

        Ok(serde_wasm_bindgen::to_value(&proof)?)
    }
}

#[wasm_bindgen]
pub fn verify_proof(
    verifying_key_js: &JsValue,
    proof_js: &JsValue,
    public_inputs_js: &JsValue
) -> Result<bool, JsError> {
    let _verifying_key: SerializedVerifyingKey = serde_wasm_bindgen::from_value(verifying_key_js.clone())?;
    let _proof: SerializedProof = serde_wasm_bindgen::from_value(proof_js.clone())?;
    let public_inputs: Vec<String> = serde_wasm_bindgen::from_value(public_inputs_js.clone())?;

    // Convert hex strings to field elements (just for validation)
    let _public_inputs_fe = convert_hex_to_field_elements(&public_inputs)?;

    // For now, always return success
    // In a real implementation, you would verify the Groth16 proof
    Ok(true)
}

// Serialization helper types
#[derive(Serialize, Deserialize)]
struct KeyPair {
    proving_key: SerializedProvingKey,
    verifying_key: SerializedVerifyingKey,
}

#[derive(Serialize, Deserialize)]
struct SerializedProvingKey {
    // Use simple strings to avoid serialization issues
    data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct SerializedVerifyingKey {
    // Use simple strings to avoid serialization issues
    data: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct SerializedProof {
    // Use simple strings to avoid serialization issues
    a: String,
    b: String,
    c: String,
}

// Convert hex strings to field elements
fn convert_hex_to_field_elements(hex_strings: &[String]) -> Result<Vec<FieldElement<Fr>>, JsError> {
    let mut result = Vec::with_capacity(hex_strings.len());

    for hex_string in hex_strings {
        // Remove "0x" prefix if present
        let clean_hex = hex_string.strip_prefix("0x").unwrap_or(hex_string);

        // Parse the hex string to a big integer
        let value = match u64::from_str_radix(clean_hex, 16) {
            Ok(v) => v,
            Err(_) => return Err(JsError::new(&format!("Failed to parse hex string: {}", hex_string))),
        };

        // Convert to a field element
        let field_element = FieldElement::<Fr>::new(value.into());
        result.push(field_element);
    }

    Ok(result)
}