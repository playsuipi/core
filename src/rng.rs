use rand::prelude::random;
pub use rand::seq::SliceRandom;
pub use rand::SeedableRng;
pub use rand_chacha::ChaCha20Rng;

/// Get Suipi game RNG from a 256 bit seed
pub fn get_seeded_rng(seed: [u8; 32]) -> ChaCha20Rng {
    ChaCha20Rng::from_seed(seed)
}

/// Get Suipi game RNG from a random seed
pub fn get_random_rng() -> ChaCha20Rng {
    get_seeded_rng(random())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_deterministic_rng() {
        let seed = [0; 32]; // Constant seed provides constant results
        let mut rng = get_seeded_rng(seed);
        assert_eq!(rng.gen_range(1..=1000000), 679211);
        assert_eq!(rng.gen_range(1..=1000000), 563446);
        assert_eq!(rng.gen_range(1..=1000000), 896155);
    }
}
