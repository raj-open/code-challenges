/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;
use rand::prelude::IndexedRandom;

use super::constants;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn roll_dice(
    rng: &mut ChaCha8Rng,
) -> Vec<String> {
    constants::DICE
    .iter()
    .map(|die| die.choose(rng).unwrap().to_string())
    .collect()
}
