use crate::profiling::Profiler;
use crate::algorithms::{
    newhope::NewHope, sphincs::Sphincs, mceliece::McEliece, sike::Sike, new_algorithm::NewAlgorithm,
    new_algorithm_advanced::AdvancedAlgorithm,
};
use std::collections::HashMap;

pub struct CryptoToolkit {
    config: Config,                          // Stores configuration settings
    cache: HashMap<String, Vec<u8>>,         // Caches intermediate results
    logs: Vec<String>,                       // Stores execution logs
    #[allow(dead_code)]
    profiling_data: HashMap<String, u128>,   // Stores profiling results
}

impl CryptoToolkit {
    pub fn new(config: Config) -> Self {
        CryptoToolkit {
            config,
            cache: HashMap::new(),
            logs: Vec::new(),
            profiling_data: HashMap::new(),
        }
    }

    /// Runs a demonstration of the available algorithms.
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

    /// Profiles the execution time of the algorithm demonstrations.
    pub fn profile_algorithms(&self) {
        self.profile_algorithm("NewHope key exchange", || {
            let newhope = NewHope::new();
            let public_key = vec![1, 2, 3, 4];
            newhope.exchange(&public_key);
        });

        self.profile_algorithm("SPHINCS+ signing", || {
            let sphincs = Sphincs::new();
            let message = vec![1, 2, 3, 4];
            sphincs.sign(&message);
        });

        self.profile_algorithm("McEliece encryption", || {
            let mceliece = McEliece::new();
            let plaintext = vec![1, 2, 3, 4];
            mceliece.encrypt(&plaintext);
        });

        self.profile_algorithm("SIKE key encapsulation", || {
            let sike = Sike::new();
            sike.encapsulate();
        });

        self.profile_algorithm("AdvancedAlgorithm encryption", || {
            let advanced_algorithm = AdvancedAlgorithm::new();
            let plaintext = vec![1, 2, 3, 4];
            advanced_algorithm.encrypt(&plaintext);
        });
    }

    /// Profiles the execution time of the new algorithm.
    pub fn profile_new_algorithm(&self) {
        self.profile_algorithm("NewAlgorithm processing", || {
            let algorithm = NewAlgorithm::new();
            let input_data = vec![1, 2, 3, 4];
            algorithm.process_data(&input_data);
        });

        self.profile_algorithm("AdvancedAlgorithm processing", || {
            let advanced_algorithm = AdvancedAlgorithm::new();
            let input_data = vec![1, 2, 3, 4];
            advanced_algorithm.process_data(&input_data);
        });
    }

    /// Helper function to profile any algorithm.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the algorithm being tested, which will be printed to the console.
    /// * `f` - A closure representing the algorithm to be profiled.
    fn profile_algorithm<F>(&self, name: &str, f: F)
    where
        F: FnOnce(),
    {
        let duration = Profiler::profile_execution_time(f);
        println!("{} execution time: {} ms", name, duration);
    }

    /// Logs a message into the toolkit's log storage.
    ///
    /// # Arguments
    ///
    /// * `message` - The log message to be stored.
    pub fn log_message(&mut self, message: &str) {
        self.logs.push(message.to_string());
    }

    /// Retrieves the current configuration.
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Updates the configuration with new settings.
    ///
    /// # Arguments
    ///
    /// * `new_config` - The new configuration settings.
    pub fn update_config(&mut self, new_config: Config) {
        self.config = new_config;
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for CryptoToolkit {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

/// Configuration settings for CryptoToolkit.
#[derive(Default)]
pub struct Config {
    // Example configuration fields
    pub security_level: u8,
    pub enable_profiling: bool,
}
