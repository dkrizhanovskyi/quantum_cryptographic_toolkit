/// Quantum-resistant cryptographic algorithms.
/// 
/// This module contains various quantum-resistant cryptographic algorithms.

pub mod newhope;
pub mod sphincs;
pub mod mceliece;
pub mod sike;
pub mod new_algorithm;

pub fn example_algorithm() -> String {
    String::from("This is an example quantum-resistant algorithm.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_algorithm() {
        let result = example_algorithm();
        assert_eq!(result, "This is an example quantum-resistant algorithm.");
    }
}
