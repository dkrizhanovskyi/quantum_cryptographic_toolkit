// src/lib.rs

/// This library provides implementations of various quantum-resistant cryptographic algorithms,
/// along with tools for profiling and demonstrating these algorithms.

pub mod algorithms;
pub mod core;
pub mod profiling;

/// A simple example function to ensure the library is working as expected.
///
/// # Returns
///
/// A string message confirming the library is operational.
pub fn example_function() -> String {
    String::from("The Quantum Cryptographic Toolkit is operational.")
}

#[cfg(test)]
mod tests {
    mod integration_test;
    mod algorithm_benchmark;

    use super::*;

    #[test]
    fn test_example_function() {
        let result = example_function();
        assert_eq!(result, "The Quantum Cryptographic Toolkit is operational.");
    }
}
