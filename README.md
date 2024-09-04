# Quantum Cryptographic Toolkit

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.13317139.svg)](https://doi.org/10.5281/zenodo.13317139)

## Description

The Quantum Cryptographic Toolkit is a collection of tools and libraries written in Rust, designed to develop, test, and deploy quantum-resistant cryptographic algorithms. It supports several well-known quantum-safe algorithms, making it a powerful resource for post-quantum cryptography.

## Project Structure

The project is structured as follows:

- **src/algorithms/**: Contains implementations of various quantum-resistant cryptographic algorithms, including:
  - **NewHope**: Lattice-based key exchange.
  - **SPHINCS+**: Stateless hash-based digital signature scheme.
  - **McEliece**: Code-based cryptosystem.
  - **SIKE**: Supersingular isogeny-based key encapsulation mechanism.
  - **FrodoKEM** and **Kyber** (located in `experimental`): Lattice-based key encapsulation algorithms.
  
- **src/profiling/**: Contains tools for profiling the performance of cryptographic algorithms.
  
- **src/core.rs**: Core library functionality providing the main interface for interacting with the algorithms and profiling tools.
  
- **src/main.rs**: Entry point for running algorithm demonstrations and profiling.

## Usage

To use the toolkit, you can initialize the core library and run demonstrations or profiling as shown below:

```rust
use quantum_cryptographic_toolkit::core::CryptoToolkit;

fn main() {
    let toolkit = CryptoToolkit::new();
    toolkit.run_algorithm_demo();
    toolkit.profile_algorithms();
}
```

### Example - Using NewHope Algorithm

Here’s a simple example that demonstrates how to use the NewHope algorithm for key exchange:

```rust
use quantum_cryptographic_toolkit::algorithms::newhope::NewHope;

fn main() {
    let newhope = NewHope::new();
    let public_key = vec![1, 2, 3, 4];
    let shared_secret = newhope.exchange(&public_key);
    println!("Shared secret: {:?}", shared_secret);
}
```

## Installation

To include the Quantum Cryptographic Toolkit in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
quantum_cryptographic_toolkit = { path = "path/to/quantum_cryptographic_toolkit" }
```

Ensure that you have all the necessary dependencies installed, and run `cargo build` to compile the project.

## Running with Docker

You can also use Docker to containerize and run the toolkit. To build and run the Docker image:

```bash
podman build -t quantum_cryptographic_toolkit .
podman run --rm -it quantum_cryptographic_toolkit
```

This will build the Docker container and run the project inside an isolated environment.

## Examples

The repository contains several examples that demonstrate how to use the various cryptographic algorithms. You can run them by using `cargo run --example <example_name>`. Examples include:

- **NewHope Example**:
  ```bash
  cargo run --example newhope_example
  ```

- **SPHINCS+ Example**:
  ```bash
  cargo run --example sphincs_example
  ```

## Contributing

Contributions are welcome! Please follow the guidelines outlined in the [CONTRIBUTING.md](CONTRIBUTING.md) file. Contributions can include bug reports, feature requests, or even improvements to the cryptographic algorithms implemented.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

