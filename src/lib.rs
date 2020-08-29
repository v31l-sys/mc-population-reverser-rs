#[macro_use]
extern crate lazy_static;

extern crate java_random_lcg;
use java_random_lcg::*;

pub mod math;
pub mod population_reverser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_test() {
        let seeds: Vec<i64> = crate::population_reverser::population_seed_utils
            ::reverse(2721704043401555507, 32, 64);

        assert!(seeds.contains(&(41823749187923 as i64)),
                format!("Failed to get seeds... returned: {:?}", seeds));
    }
}
