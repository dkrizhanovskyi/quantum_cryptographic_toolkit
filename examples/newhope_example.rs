// examples/newhope_example.rs

use quantum_cryptographic_toolkit::algorithms::newhope::NewHope;

/// Demonstrates the use of the NewHope algorithm for key exchange.
fn main() {
    let newhope = NewHope::new();

    let public_key = vec![1, 2, 3, 4];
    println!("Public Key: {:?}", &public_key);

    let shared_secret = newhope.exchange(&public_key);
    println!("Shared Secret: {:?}", &shared_secret);

    // In a real application, the shared secret would be used for secure communication.
    println!("Key exchange using NewHope was successful!");
}
