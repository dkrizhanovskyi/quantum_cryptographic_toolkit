// examples/mceliece_example.rs

use quantum_cryptographic_toolkit::algorithms::mceliece::McEliece;

/// Demonstrates the use of the McEliece algorithm for encryption and decryption.
fn main() {
    let mceliece = McEliece::new();

    let plaintext = vec![10, 20, 30, 40];
    println!("Plaintext: {:?}", &plaintext);

    let ciphertext = mceliece.encrypt(&plaintext);
    println!("Encrypted: {:?}", &ciphertext);

    let decrypted_text = mceliece.decrypt(&ciphertext);
    println!("Decrypted: {:?}", &decrypted_text);

    assert_eq!(decrypted_text, plaintext);
    println!("Encryption and decryption were successful!");
}
