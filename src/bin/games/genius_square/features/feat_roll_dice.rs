// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;

use crate::models::dice::roll_dice;
use crate::models::dice::Die;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

/// Feature to roll the dice
pub fn feature_roll_dice(rng: &mut ChaCha8Rng, option_roll: Option<Vec<String>>) -> Vec<Die> {
    let mut faces = option_roll.unwrap_or_else(|| roll_dice(rng));
    faces.sort();
    let dice: Vec<Die> = faces.iter().map(Die::from_string).collect();
    println!("\nRoll: {}", faces.join(" "));

    dice
}
