### API Reference

# Quantum Cryptographic Toolkit

The Quantum Cryptographic Toolkit is a library designed to provide a suite of quantum-resistant cryptographic algorithms. This API reference outlines the key components, their usage, and examples.

---

## Table of Contents

- [CryptoToolkit](#cryptotoolkit)
- [Algorithms](#algorithms)
  - [NewHope](#newhope)
  - [SPHINCS+](#sphincs)
  - [McEliece](#mceliece)
  - [SIKE](#sike)
  - [FrodoKEM](#frodokem)
  - [Kyber](#kyber)
- [Utilities](#utilities)
  - [log_message](#log_message)
  - [clear_cache](#clear_cache)

---

## CryptoToolkit

The `CryptoToolkit` is the core structure that provides access to various quantum-resistant cryptographic algorithms and utility functions.

### Methods

- **new()**: Initializes a new instance of `CryptoToolkit`.
  
  ```rust
  let toolkit = CryptoToolkit::new();
  ```

- **run_algorithm_demo()**: Demonstrates the usage of different algorithms in the toolkit.

- **profile_algorithms()**: Profiles the execution time of various algorithms.

- **profile_new_algorithm()**: Specifically profiles newly added algorithms.

---

## Algorithms

### NewHope

The NewHope algorithm is a lattice-based key exchange mechanism that provides post-quantum security.

#### Methods

- **new()**: Initializes a new instance of the NewHope algorithm.
  
  ```rust
  let newhope = NewHope::new();
  ```

- **exchange(public_key: &[u8]) -> Vec<u8>**: Executes the key exchange process.

### SPHINCS+

SPHINCS+ is a stateless hash-based signature scheme that offers strong security against quantum attacks.

#### Methods

- **new()**: Initializes a new instance of the SPHINCS+ algorithm.
  
  ```rust
  let sphincs = Sphincs::new();
  ```

- **sign(message: &[u8]) -> Vec<u8>**: Signs the provided message.

- **verify(message: &[u8], signature: &[u8]) -> bool**: Verifies the signature against the provided message.

### McEliece

McEliece is a code-based cryptographic system known for its strong resistance against quantum attacks.

#### Methods

- **new()**: Initializes a new instance of the McEliece algorithm.
  
  ```rust
  let mceliece = McEliece::new();
  ```

- **encrypt(plaintext: &[u8]) -> Vec<u8>**: Encrypts the given plaintext.

- **decrypt(ciphertext: &[u8]) -> Vec<u8>**: Decrypts the provided ciphertext.

### SIKE

SIKE (Supersingular Isogeny Key Encapsulation) is an isogeny-based encryption algorithm.

#### Methods

- **new()**: Initializes a new instance of the SIKE algorithm.
  
  ```rust
  let sike = Sike::new();
  ```

- **encapsulate() -> Vec<u8>**: Encapsulates a key using the SIKE algorithm.

- **decapsulate(encapsulated_key: &[u8]) -> Vec<u8>**: Decapsulates the key.

### FrodoKEM

FrodoKEM is a lattice-based key encapsulation mechanism that provides quantum-resistant security.

#### Methods

- **new()**: Initializes a new instance of the FrodoKEM algorithm.
  
  ```rust
  let frodokem = FrodoKEM::new();
  ```

- **encapsulate() -> Vec<u8>**: Encapsulates a key using the FrodoKEM algorithm.

- **decapsulate(encapsulated_key: &[u8]) -> Vec<u8>**: Decapsulates the key.

### Kyber

Kyber is another lattice-based key encapsulation mechanism, known for its efficiency and strong security.

#### Methods

- **new()**: Initializes a new instance of the Kyber algorithm.
  
  ```rust
  let kyber = Kyber::new();
  ```

- **encapsulate() -> Vec<u8>**: Encapsulates a key using the Kyber algorithm.

- **decapsulate(encapsulated_key: &[u8]) -> Vec<u8>**: Decapsulates the key.

---

## Utilities

### log_message

Logs a message within the toolkit's internal logging system.

```rust
log_message(&mut logs, "This is a log message.");
```

### clear_cache

Clears the cache used by the toolkit.

```rust
clear_cache(&mut cache);
```

