use serde_json::Value;
use std::fs;
use std::path::Path;

pub mod stark_proof {
    use super::*;

    /// Generates a proof for the given secret(dummy-implementation: by looking it up in a JSON file).
    ///
    /// # Arguments
    /// * `secret` - The secret for which the proof is to be generated.
    ///
    /// # Returns
    /// * `Ok(String)` - JSON string representing the proof if successfully found.
    /// * `Err(String)` - Error message if the proof generation fails.
    pub fn generate_proof_of_secret(secret: &str) -> Result<String, String> {
        // Define the path to the JSON file containing proofs.
        let proof_file_path = Path::new("examples/proof.json");

        // Read and parse the JSON file.
        let parsed_proofs: Value = fs::read_to_string(proof_file_path)
            .map_err(|_| "Failed to read the proof file".to_string())
            .and_then(|data| {
                serde_json::from_str(&data)
                    .map_err(|_| "Failed to parse the proof file as valid JSON".to_string())
            })?;

        // Retrieve the proof corresponding to the secret.
        parsed_proofs
            .get(secret)
            .map(|proof| proof.to_string())
            .ok_or_else(|| "Proof not found for the given secret".to_string())
    }
}
