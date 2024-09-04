// src/profiling.rs

use std::time::Instant;

/// Profiler is a utility for measuring the execution time of algorithms and functions.
pub struct Profiler;

impl Profiler {
    /// Measures the execution time of a given function or closure.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure or function whose execution time is to be measured.
    ///
    /// # Returns
    ///
    /// The execution time in milliseconds as a `u128`.
    pub fn profile_execution_time<F>(f: F) -> u128
    where
        F: FnOnce(),
    {
        let start_time = Instant::now();
        f();
        start_time.elapsed().as_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_execution_time() {
        let duration = Profiler::profile_execution_time(|| {
            let mut sum: i64 = 0;
            for i in 0..1_000_000_i64 {  // Use i64 for larger numbers
                sum += i;
            }
            assert_eq!(sum, 499999500000_i64);  // Use i64 for large literal
        });

        // The comparison for duration >= 0 is unnecessary, so it's removed
        println!("Measured duration: {} ms", duration);
    }
}
