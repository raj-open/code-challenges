// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;

use crate::models::dice::roll_dice;
use crate::models::dice::Die;
use crate::models::constants::EnumPiece;
use crate::models::pieces::Piece;
use crate::models::board::GameBoard;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

/// Feature to set up the game
pub fn feature_initialise_game(
    rng: &mut ChaCha8Rng,
    option_roll: Option<Vec<String>>,
) -> GameBoard {
    // Roll the dice
    let mut faces = option_roll.unwrap_or_else(|| roll_dice(rng));
    faces.sort();
    let dice: Vec<Die> = faces.iter().map(Die::from_string).collect();
    println!("\nRoll: {}", faces.join(" "));

    // Establish the problem
    let coords = dice.iter().map(|die| die.to_coords()).collect();
    let block = Piece::from_coords(coords, Some(EnumPiece::Block));
    let board = GameBoard::new(&block);
    println!("\nProblem:\n{}", board.pretty());

    board
}
