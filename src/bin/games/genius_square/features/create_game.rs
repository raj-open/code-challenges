/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;

use crate::models::dice::methods::roll_dice;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn feature_create_game(
    rng: &mut ChaCha8Rng,
    option_roll: Option<Vec<String>>,
) {
    let roll: Vec<String>;
    match option_roll {
        Some(x) => {
            roll = x;
        },
        None => {
            roll = roll_dice(rng);
        }
    }
    println!("\nRoll: {}.\n", roll.join(", "));
}
