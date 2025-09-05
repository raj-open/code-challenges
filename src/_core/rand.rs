/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

#[allow(unused)]
pub fn seed_rng(x: Option<String>) -> ChaCha8Rng {
    match x {
        Some(seed_str) => {
            // string -> bytes
            let mut seed = [0u8; 32];
            let seed_bytes = seed_str.as_bytes();
            let len = seed_bytes.len().min(32);
            seed[..len].copy_from_slice(&seed_bytes[..len]);

            // create RNG
            let rng = ChaCha8Rng::from_seed(seed);
            return rng;
        },
        None => {
            return ChaCha8Rng::from_os_rng();
        },
    }
}
