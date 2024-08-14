#[cfg(test)]
mod tests {
    use crate::algorithms::{mceliece::McEliece, newhope::NewHope, sphincs::Sphincs, sike::Sike, new_algorithm::NewAlgorithm};

    #[test]
    fn test_mceliece_empty_input() {
        let mceliece = McEliece::new();
        let plaintext = vec![];
        let ciphertext = mceliece.encrypt(&plaintext);
        let decrypted_text = mceliece.decrypt(&ciphertext);
        assert_eq!(decrypted_text, plaintext, "Decryption failed for empty input");
    }

    #[test]
    fn test_newhope_max_key_length() {
        let newhope = NewHope::new();
        let public_key = vec![255; 1024];  // Simulate a large public key
        let shared_secret = newhope.exchange(&public_key);
        assert_eq!(shared_secret.len(), public_key.len(), "Shared secret length mismatch with max key length");
    }

    #[test]
    fn test_sphincs_invalid_signature() {
        let sphincs = Sphincs::new();
        let message = vec![1, 2, 3, 4];
        let signature = vec![255; 64]; // Simulate an invalid signature
        let is_valid = sphincs.verify(&message, &signature);
        assert!(!is_valid, "Verification should fail for an invalid signature");
    }

    #[test]
    fn test_sike_edge_case_encapsulation() {
        let sike = Sike::new();
        let encapsulated_key = sike.encapsulate();
        let decapsulated_key = sike.decapsulate(&encapsulated_key);
        assert_eq!(decapsulated_key, encapsulated_key, "Decapsulation failed for edge case");
    }

    #[test]
    fn test_new_algorithm_large_input() {
        let algorithm = NewAlgorithm::new();
        let input_data = vec![1; 10000];  // Simulate a large input data
        let output_data = algorithm.process_data(&input_data);
        assert_eq!(output_data.len(), input_data.len(), "Processing failed for large input data");
    }
}
