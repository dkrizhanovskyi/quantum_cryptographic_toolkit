/// Quantum Cryptographic Toolkit library.
/// 
/// This module provides the foundation for quantum-resistant cryptographic algorithms.

pub mod algorithms;
pub mod profiling;
pub mod core;

pub fn toolkit_info() -> String {
    String::from("Quantum Cryptographic Toolkit v0.1.0")
}
