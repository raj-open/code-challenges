/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use rand_chacha::ChaCha8Rng;
use std::time::Duration;
use std::time::SystemTime;

use crate::models::dice::methods::roll_dice;
use crate::models::dice::models::Die;
use crate::models::constants::enums::EnumPiece;
use crate::models::pieces::models::Piece;
use crate::models::board::models::GameBoard;
use crate::algorithms::solve::solve_brute_force;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

const TIMEOUT: Duration = Duration::from_secs(10);

pub fn feature_setup_game(
    rng: &mut ChaCha8Rng,
    option_roll: Option<Vec<String>>,
) {
    // Roll the dice
    let faces = option_roll.unwrap_or_else(|| roll_dice(rng));
    let dice: Vec<Die> = faces.iter()
        .map(|face| Die::from_string(face))
        .collect();
    println!("\nRoll: {}.", faces.join(", "));

    // Establish the problem
    let coords = dice.iter().map(|die| die.to_coords()).collect();
    let block = Piece::from_coords(coords, Some(EnumPiece::Block));
    let board = GameBoard::new(&block);
    println!("\nProblem:\n{}", board.pretty());

    // Solve the problem
    print!("\nCompute solution ... ");
    let rx = solve_brute_force(&board, true);
    let mut solution: Option<GameBoard> = None;
    let mut dt: Option<Duration> = None;
    let mut n = 0;
    let time = SystemTime::now();

    while let Ok(board) = rx.recv_timeout(TIMEOUT) {
        if n == 0 {
            dt = Some(time.elapsed().unwrap());
            solution = Some(board);
        }
        n += 1;
    }

    let dt_total = time.elapsed().unwrap();
    let dt_mean: Duration = if n > 0 {dt_total/n} else {Duration::from_secs(0)};
    let dt = dt.unwrap_or(dt_total);
    match solution {
        Some(board) => {
            println!("found {n} solutions.");
            println!("Time for 1st solution:      {dt:2?}");
            println!("Average time per solution:  {dt_mean:2?}");
            println!("Total time:                 {dt_total:2?}");
            println!("\nSolution 1:\n{}\n", board.pretty());
        },
        None => {
            println!("\x1b[91mno solution found!\x1b[0m\n");
        }
    }
}
