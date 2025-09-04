/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;
use rand::prelude::IndexedRandom;

use crate::models::constants::dice::*;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn roll_dice(
    rng: &mut ChaCha8Rng,
) -> Vec<String> {
    DICE
    .iter()
    .map(|die| die.choose(rng).unwrap().to_string())
    .collect()
}
