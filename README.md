   # Quantum Cryptographic Toolkit

   [![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.13317139.svg)](https://doi.org/10.5281/zenodo.13317139)

   ## Description
   The Quantum Cryptographic Toolkit is a collection of tools and libraries written in Rust, designed to develop, test, and deploy quantum-resistant cryptographic algorithms.

   ## Project Structure
   The project is structured as follows:

   - **src/algorithms/**: Contains implementations of various quantum-resistant cryptographic algorithms, including:
     - NewHope (Lattice-based)
     - SPHINCS+ (Hash-based)
     - McEliece (Code-based)
     - SIKE (Isogeny-based)
   - **src/profiling/**: Contains tools for profiling the performance of the cryptographic algorithms.
   - **src/core.rs**: Core library functionality, providing the main interface for interacting with the algorithms and profiling tools.
   - **src/main.rs**: Entry point for running demonstrations and profiling.

   ## Usage
   To use the toolkit, you can initialize the core library and run demonstrations or profiling like this:

   ```rust
   use quantum_cryptographic_toolkit::core::CryptoToolkit;

   fn main() {
       let toolkit = CryptoToolkit::new();
       toolkit.run_algorithm_demo();
       toolkit.profile_algorithms();
   }
   ```

   ## Installation
   To include the Quantum Cryptographic Toolkit in your project, add the following to your `Cargo.toml`:

   ```toml
   [dependencies]
   quantum_cryptographic_toolkit = { path = "path/to/quantum_cryptographic_toolkit" }
   ```

   ## Examples
   Here is a basic example of using the NewHope algorithm:

   ```rust
   use quantum_cryptographic_toolkit::algorithms::newhope::NewHope;

   fn main() {
       let newhope = NewHope::new();
       let public_key = vec![1, 2, 3, 4];
       let shared_secret = newhope.exchange(&public_key);
       println!("Shared secret: {:?}", shared_secret);
   }
   ```

   ## Contributing
   Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

   ## License
   This project is licensed under the MIT LICENSE - see the [LICENSE](LICENSE) file for details.
