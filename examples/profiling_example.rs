use quantum_cryptographic_toolkit::core::CryptoToolkit;

fn main() {
    let toolkit = CryptoToolkit::default();

    toolkit.profile_algorithms();
    toolkit.profile_new_algorithm();
}
