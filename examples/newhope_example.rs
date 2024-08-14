use quantum_cryptographic_toolkit::algorithms::newhope::NewHope;

fn main() {
    let newhope = NewHope::new();
    let public_key = vec![1, 2, 3, 4];
    let shared_secret = newhope.exchange(&public_key);
    println!("Shared secret: {:?}", shared_secret);
}
