use std::time::Instant;


pub struct Profiler;

impl Profiler {

    pub fn profile_execution_time<F>(f: F) -> u128
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        f();
        start.elapsed().as_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_execution_time() {
        let duration = Profiler::profile_execution_time(|| {
            let sum: u32 = (0..1000).sum();
            assert_eq!(sum, 499500);
        });
        println!("Execution time: {} ms", duration);
    }
}
