use quantum_cryptographic_toolkit::core::CryptoToolkit;

fn main() {
    // Initialize the toolkit
    let toolkit = CryptoToolkit::new();

    // Run a demonstration of the existing algorithms
    toolkit.run_algorithm_demo();

    // Profile the execution time of the existing algorithms
    toolkit.profile_algorithms();

    // Profile the execution time of the new algorithm
    toolkit.profile_new_algorithm();
}
