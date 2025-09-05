/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;

use crate::models::dice::methods::roll_dice;
use crate::models::dice::models::Die;
use crate::models::constants::enums::EnumPiece;
use crate::models::pieces::models::Piece;
use crate::models::board::models::GameBoard;
use crate::algorithms::solve::solve_brute_force;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

pub fn feature_setup_game(
    rng: &mut ChaCha8Rng,
    option_roll: Option<Vec<String>>,
) {
    // Roll the dice
    let faces = option_roll.unwrap_or_else(|| roll_dice(rng));
    let dice: Vec<Die> = faces.iter()
        .map(|face| Die::from_string(face))
        .collect();
    println!("\nRoll: {}.\n", faces.join(", "));

    // Establish the problem
    let coords = dice.iter().map(|die| die.to_coords()).collect();
    let block = Piece::from_coords(coords, Some(EnumPiece::Block));
    let board = GameBoard::new(&block);
    println!("\nProblem:\n{}", board.pretty());

    // Solve the problem
    println!("\nCompute solution ...\n");
    let rx = solve_brute_force(&board);
    let mut n = 0;
    #[allow(while_true)]
    while true {
        match rx.recv() {
            Ok((dt, board)) => {
                n += 1;
                println!("... completed in {:.2?}", dt);
                println!("\nSolution {n}:\n{}\n", board.pretty());
                // NOTE: Currently stop on first solution
                break;
            },
            Err(_) => {
                break;
            }
        }
    }
    if n == 0 {
        println!("\n\x1b[91mNo solution found!\x1b[0m\n");
    }
}
