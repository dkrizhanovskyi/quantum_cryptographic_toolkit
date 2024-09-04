// src/tests/integration_test.rs

use crate::algorithms::{newhope::NewHope, sphincs::Sphincs, mceliece::McEliece, sike::Sike, new_algorithm::NewAlgorithm, new_algorithm_advanced::AdvancedAlgorithm};

/// Integration test for the NewHope algorithm's key exchange and shared secret consistency.
#[test]
fn test_newhope_integration() {
    let newhope = NewHope::new();
    let public_key = vec![1, 2, 3, 4];
    let shared_secret_1 = newhope.exchange(&public_key);
    let shared_secret_2 = newhope.exchange(&public_key);
    assert_eq!(shared_secret_1, shared_secret_2, "Shared secrets should be consistent.");
}

/// Integration test for the SPHINCS+ algorithm's signing and verification.
#[test]
fn test_sphincs_integration() {
    let sphincs = Sphincs::new();
    let message = vec![1, 2, 3, 4];
    let signature = sphincs.sign(&message);
    let is_valid = sphincs.verify(&message, &signature);
    assert!(is_valid, "Signature should be valid for the original message.");
}

/// Integration test for the McEliece algorithm's encryption and decryption.
#[test]
fn test_mceliece_integration() {
    let mceliece = McEliece::new();
    let plaintext = vec![10, 20, 30, 40];
    let ciphertext = mceliece.encrypt(&plaintext);
    let decrypted_text = mceliece.decrypt(&ciphertext);
    assert_eq!(decrypted_text, plaintext, "Decrypted text should match the original plaintext.");
}

/// Integration test for the SIKE algorithm's encapsulation and decapsulation.
#[test]
fn test_sike_integration() {
    let sike = Sike::new();
    let encapsulated_key = sike.encapsulate();
    let decapsulated_key = sike.decapsulate(&encapsulated_key);
    assert_eq!(decapsulated_key, sike.public_key, "Decapsulated key should match the original public key.");
}

/// Integration test for the NewAlgorithm's data processing.
#[test]
fn test_new_algorithm_integration() {
    let algorithm = NewAlgorithm::new();
    let input_data = vec![5, 10, 15, 20];
    let processed_data = algorithm.process_data(&input_data);
    let expected_output: Vec<u8> = input_data.iter().map(|&x| x.wrapping_add(1)).collect();
    assert_eq!(processed_data, expected_output, "Processed data should match the expected transformation.");
}

/// Integration test for the AdvancedAlgorithm's encryption and decryption.
#[test]
fn test_advanced_algorithm_integration() {
    let algorithm = AdvancedAlgorithm::new();
    let plaintext = vec![11, 22, 33, 44];
    let ciphertext = algorithm.encrypt(&plaintext);
    let decrypted_text = algorithm.decrypt(&ciphertext);
    assert_eq!(decrypted_text, plaintext, "Decrypted text should match the original plaintext.");
}
