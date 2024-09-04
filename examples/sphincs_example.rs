// examples/sphincs_example.rs

use quantum_cryptographic_toolkit::algorithms::sphincs::Sphincs;

/// Demonstrates the use of the SPHINCS+ algorithm for signing and verification.
fn main() {
    let sphincs = Sphincs::new();

    let message = vec![1, 2, 3, 4];
    println!("Message: {:?}", &message);

    let signature = sphincs.sign(&message);
    println!("Signature: {:?}", &signature);

    let is_valid = sphincs.verify(&message, &signature);
    println!("Is the signature valid? {}", is_valid);

    assert!(is_valid, "The signature should be valid.");
    println!("Message signing and verification using SPHINCS+ were successful!");
}
