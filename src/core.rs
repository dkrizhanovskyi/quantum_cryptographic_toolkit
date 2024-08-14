use crate::profiling::Profiler;
use crate::algorithms::{newhope::NewHope, sphincs::Sphincs, mceliece::McEliece, sike::Sike, new_algorithm::NewAlgorithm, new_algorithm_advanced::AdvancedAlgorithm};

pub struct CryptoToolkit {
    // Core properties or state could be added here if needed later
}

impl CryptoToolkit {
    /// Initialize the toolkit with default settings or configurations.
    pub fn new() -> Self {
        CryptoToolkit {
            // Initialization logic can be extended as needed
        }
    }

    /// Run a demonstration of the available algorithms.
    pub fn run_algorithm_demo(&self) {
        let newhope = NewHope::new();
        let sphincs = Sphincs::new();

        // Demonstrate NewHope key exchange
        let public_key = vec![1, 2, 3, 4];
        let shared_secret = newhope.exchange(&public_key);
        println!("NewHope shared secret: {:?}", shared_secret);

        // Demonstrate SPHINCS+ signature creation
        let message = vec![1, 2, 3, 4];
        let signature = sphincs.sign(&message);
        println!("SPHINCS+ signature: {:?}", signature);
    }

    /// Profile the execution time of the algorithms' demonstrations.
    pub fn profile_algorithms(&self) {
        // Profile NewHope key exchange
        let newhope_duration = Profiler::profile_execution_time(|| {
            let newhope = NewHope::new();
            let public_key = vec![1, 2, 3, 4];
            let _shared_secret = newhope.exchange(&public_key);
        });
        println!("NewHope key exchange execution time: {} ms", newhope_duration);

        // Profile SPHINCS+ signing
        let sphincs_duration = Profiler::profile_execution_time(|| {
            let sphincs = Sphincs::new();
            let message = vec![1, 2, 3, 4];
            let _signature = sphincs.sign(&message);
        });
        println!("SPHINCS+ signing execution time: {} ms", sphincs_duration);

        // Profile McEliece encryption
        let mceliece_duration = Profiler::profile_execution_time(|| {
            let mceliece = McEliece::new();
            let plaintext = vec![1, 2, 3, 4];
            let _ciphertext = mceliece.encrypt(&plaintext);
        });
        println!("McEliece encryption execution time: {} ms", mceliece_duration);

        // Profile SIKE key encapsulation
        let sike_duration = Profiler::profile_execution_time(|| {
            let sike = Sike::new();
            let _encapsulated_key = sike.encapsulate();
        });
        println!("SIKE key encapsulation execution time: {} ms", sike_duration);

        // Profile AdvancedAlgorithm encryption
        let advanced_algorithm_duration = Profiler::profile_execution_time(|| {
            let advanced_algorithm = AdvancedAlgorithm::new();
            let plaintext = vec![1, 2, 3, 4];
            let _ciphertext = advanced_algorithm.encrypt(&plaintext);
        });
        println!("AdvancedAlgorithm encryption execution time: {} ms", advanced_algorithm_duration);
    }

    /// Profile the execution time of the new algorithm.
    pub fn profile_new_algorithm(&self) {
        let new_algorithm_duration = Profiler::profile_execution_time(|| {
            let algorithm = NewAlgorithm::new();
            let input_data = vec![1, 2, 3, 4];
            let _output_data = algorithm.process_data(&input_data);
        });
        println!("NewAlgorithm processing execution time: {} ms", new_algorithm_duration);

        let advanced_algorithm_duration = Profiler::profile_execution_time(|| {
            let advanced_algorithm = AdvancedAlgorithm::new();
            let input_data = vec![1, 2, 3, 4];
            let _output_data = advanced_algorithm.process_data(&input_data);
        });
        println!("AdvancedAlgorithm processing execution time: {} ms", advanced_algorithm_duration);
    }
}

impl Default for CryptoToolkit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolkit_initialization() {
        let toolkit = CryptoToolkit::new();
        assert!(true); // Basic test to confirm initialization
    }

    #[test]
    fn test_new_algorithm_profiling() {
        let toolkit = CryptoToolkit::new();
        toolkit.profile_new_algorithm();
    }

    #[test]
    fn test_advanced_algorithm_demo() {
        let toolkit = CryptoToolkit::new();
        toolkit.run_algorithm_demo();
    }
}
