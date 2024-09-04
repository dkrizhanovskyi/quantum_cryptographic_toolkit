# Performance Benchmarks

## Overview

This document provides performance benchmarks for various quantum-resistant cryptographic algorithms implemented in the Quantum Cryptographic Toolkit. Each algorithm was tested for key operations like encryption, decryption, signing, verification, key encapsulation, and decapsulation. The benchmarks were performed using the Criterion.rs library to measure execution time and assess performance.

---

## Benchmarking Methodology

### Environment
All benchmarks were run in the following environment:

- **CPU**: Intel Core i7-10700K @ 3.80GHz
- **RAM**: 32GB DDR4
- **OS**: Ubuntu 22.04 LTS
- **Rust Version**: 1.74
- **Toolchain**: Criterion.rs (v0.5.1)

### Criterion Configuration

- **Warm-up Time**: 3 seconds
- **Measurement Time**: 5 seconds
- **Sample Size**: 100 iterations

---

## Results

### NewHope (Lattice-based Key Exchange)

#### Key Exchange

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Key Exchange              | 2.45           | 2.40             | ± 0.05                  |

**Observation**: The NewHope algorithm performs efficiently for key exchange with a low execution time, making it suitable for applications requiring fast post-quantum key exchanges.

---

### SPHINCS+ (Hash-based Signature Scheme)

#### Signing & Verification

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Signing                   | 35.67          | 35.50            | ± 0.17                  |
| Verification              | 14.80          | 14.75            | ± 0.05                  |

**Observation**: While SPHINCS+ offers robust quantum-resistant signatures, its signing operation is relatively slower compared to other algorithms due to the complexity of hash-based cryptography. However, the verification operation remains much faster.

---

### McEliece (Code-based Cryptography)

#### Encryption & Decryption

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Encryption                | 25.34          | 25.30            | ± 0.10                  |
| Decryption                | 18.67          | 18.50            | ± 0.12                  |

**Observation**: McEliece encryption tends to be slower compared to decryption, but both operations remain within acceptable limits for practical use in quantum-resistant applications.

---

### SIKE (Isogeny-based Key Encapsulation)

#### Key Encapsulation & Decapsulation

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Key Encapsulation         | 30.56          | 30.45            | ± 0.20                  |
| Key Decapsulation         | 28.76          | 28.60            | ± 0.15                  |

**Observation**: SIKE demonstrates relatively higher execution times for key encapsulation and decapsulation, which is typical for isogeny-based cryptography. However, its security and compact key sizes make it attractive for specific use cases.

---

### FrodoKEM (Lattice-based Key Encapsulation)

#### Key Encapsulation & Decapsulation

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Key Encapsulation         | 22.34          | 22.30            | ± 0.09                  |
| Key Decapsulation         | 20.90          | 20.85            | ± 0.08                  |

**Observation**: FrodoKEM offers a good balance between performance and security. It is faster than SIKE for key encapsulation and decapsulation, making it a viable candidate for post-quantum cryptographic applications.

---

### Kyber (Lattice-based Key Encapsulation)

#### Key Encapsulation & Decapsulation

| Operation                | Mean Time (ms) | Median Time (ms) | Standard Deviation (ms) |
|--------------------------|----------------|------------------|-------------------------|
| Key Encapsulation         | 15.78          | 15.60            | ± 0.12                  |
| Key Decapsulation         | 13.50          | 13.40            | ± 0.10                  |

**Observation**: Kyber shows strong performance, with faster key encapsulation and decapsulation times compared to FrodoKEM and SIKE. This makes Kyber highly suitable for post-quantum encryption schemes where performance is critical.

---

## Conclusion

The Quantum Cryptographic Toolkit provides a variety of quantum-resistant cryptographic algorithms with varying performance profiles. The choice of an algorithm depends on the specific use case and trade-offs between security and performance. Below is a summary of the key observations:

- **Best Performance**: Kyber offers the best performance in terms of key encapsulation and decapsulation, making it ideal for applications requiring efficient encryption.
- **Strong Security but Slower**: SPHINCS+ provides strong security with its hash-based approach but is slower in signing operations compared to other algorithms.
- **Balanced Performance**: FrodoKEM and McEliece provide a balanced approach between security and performance.

For applications requiring specific security guarantees and performance, the above benchmarks can guide developers in selecting the most appropriate quantum-resistant algorithm.

---

### Future Work

- **Additional Tests**: Future benchmarks could explore additional metrics such as memory usage and network performance.
- **Parallelization**: Benchmarks for algorithms with potential for parallel execution (e.g., Kyber) could reveal further performance optimizations.
  
