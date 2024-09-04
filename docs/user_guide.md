# Quantum Cryptographic Toolkit - User Guide

## Introduction

The **Quantum Cryptographic Toolkit** provides a collection of quantum-resistant cryptographic algorithms to secure communication in a post-quantum world. This guide explains how to install, run, and use the toolkit to demonstrate key cryptographic operations like key encapsulation, encryption, and digital signatures.

---

## Table of Contents

- [Installation](#installation)
- [Running the Application](#running-the-application)
- [Using the Toolkit](#using-the-toolkit)
  - [Key Algorithms](#key-algorithms)
  - [Algorithm Demonstrations](#algorithm-demonstrations)
- [Using the CLI](#using-the-cli)
- [Benchmarking](#benchmarking)
- [Troubleshooting](#troubleshooting)
  
---

## Installation

You can run the toolkit either by building it from source using Rust or by using the provided Docker image.

### Prerequisites

- **Rust**: Install the Rust toolchain by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Podman/Docker**: Make sure you have either Docker or Podman installed.

### Option 1: Building from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/dkrizhanovskyi/quantum_cryptographic_toolkit.git
   cd quantum_cryptographic_toolkit
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the application:

   ```bash
   ./target/release/quantum_cryptographic_toolkit
   ```

### Option 2: Running with Docker

1. Build the Docker image:

   ```bash
   podman build -t quantum_cryptographic_toolkit .
   ```

2. Run the Docker container:

   ```bash
   podman run --rm -it quantum_cryptographic_toolkit
   ```

---

## Running the Application

Once the toolkit is built or the container is running, you can start experimenting with different algorithms and utilities.

### Algorithm Demonstrations

The toolkit includes several cryptographic algorithms designed to be quantum-resistant. Each algorithm can be demonstrated using the built-in examples:

- **NewHope**: Lattice-based key exchange.
- **SPHINCS+**: Stateless hash-based signature scheme.
- **McEliece**: Code-based encryption system.
- **SIKE**: Supersingular isogeny key encapsulation.
- **FrodoKEM**: Lattice-based key encapsulation.
- **Kyber**: Efficient lattice-based encryption.

### Running Demos

To run demos, you can use the provided examples:

```bash
cargo run --example newhope_example
cargo run --example sphincs_example
```

Each demo demonstrates how the algorithms work by generating keys, performing encryption, signing, and verifying signatures.

---

## Using the Toolkit

### Key Algorithms

1. **NewHope**:
   - Designed for post-quantum key exchange using lattice-based cryptography.
   
   Example usage:
   ```rust
   let newhope = NewHope::new();
   let public_key = vec![1, 2, 3, 4];
   let shared_secret = newhope.exchange(&public_key);
   ```

2. **SPHINCS+**:
   - Stateless hash-based digital signature algorithm.
   
   Example usage:
   ```rust
   let sphincs = Sphincs::new();
   let signature = sphincs.sign(&message);
   let is_valid = sphincs.verify(&message, &signature);
   ```

3. **McEliece**:
   - Code-based cryptosystem for encryption and decryption.
   
   Example usage:
   ```rust
   let mceliece = McEliece::new();
   let ciphertext = mceliece.encrypt(&plaintext);
   let decrypted_text = mceliece.decrypt(&ciphertext);
   ```

For details on each algorithm, see [API Reference](./api_reference.md).

---

## Using the CLI

The toolkit also provides a command-line interface (CLI) for easy interaction. After building the project, you can execute various operations from the command line.

### Commands

- **Run Algorithm Demo**:
  ```bash
  ./quantum_cryptographic_toolkit demo
  ```

- **Profile Algorithms**:
  ```bash
  ./quantum_cryptographic_toolkit profile
  ```

- **Help**:
  To see all available commands, run:
  ```bash
  ./quantum_cryptographic_toolkit --help
  ```

---

## Benchmarking

The toolkit provides built-in benchmarks for measuring the performance of different cryptographic algorithms.

### Running Benchmarks

To run the benchmarks, use:

```bash
cargo bench
```

This will execute the performance tests and generate detailed results for each algorithm. You can find benchmark results in the `target/criterion` folder. See [Performance Benchmarks](./performance_benchmarks.md) for detailed benchmark results.

---

## Troubleshooting

### Common Issues

1. **Build Errors**:
   - Ensure that you are using the correct version of Rust (1.74 or higher).
   - If using Docker, ensure you have a compatible version of `glibc` (or use the provided `Dockerfile` with musl for static builds).

2. **Missing Dependencies**:
   - Run `cargo update` to update dependencies.
   - Ensure your environment is set up correctly with Rust and Docker.

3. **Performance Issues**:
   - If performance seems slow, ensure the system has adequate resources (CPU, RAM).
   - Use the `cargo bench` command to profile the toolkit's performance.

For additional support or to report issues, refer to the project's GitHub repository.

---

## Conclusion

This user guide outlines the steps to install, run, and use the Quantum Cryptographic Toolkit. By following these instructions, you can explore quantum-resistant cryptographic algorithms and benchmark their performance. If you encounter any issues, refer to the troubleshooting section or consult the project documentation for further details.

