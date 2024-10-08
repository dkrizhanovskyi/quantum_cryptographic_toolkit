use quantum_cryptographic_toolkit::core::CryptoToolkit;
use quantum_cryptographic_toolkit::example_function;

fn main() {
    println!("{}", example_function());

    let toolkit = CryptoToolkit::default();

    toolkit.run_algorithm_demo();
    toolkit.profile_algorithms();
    toolkit.profile_new_algorithm();
}
