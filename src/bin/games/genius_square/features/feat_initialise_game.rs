// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::models::dice::Die;
use crate::models::constants::EnumPiece;
use crate::models::pieces::Piece;
use crate::models::board::GameBoard;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

/// Feature to set up the game
pub fn feature_initialise_game(dice: &Vec<Die>) -> GameBoard {
    // Establish the problem
    let coords = dice.iter().map(|die| die.to_coords()).collect();
    let block = Piece::from_coords(coords, Some(EnumPiece::Block));
    let board = GameBoard::new(&block);
    println!("\nProblem:\n{}", board.pretty());

    board
}
