// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use std::time::Duration;
use std::time::SystemTime;

use crate::models::board::GameBoard;
use crate::algorithms::solve::solve_brute_force;
use super::constants::TIMEOUT;

// ----------------------------------------------------------------
// METHODS
// ----------------------------------------------------------------

/// Feature to solve the problem
pub fn feature_solve_game(board: &GameBoard) -> Option<GameBoard> {
    print!("\nCompute solution ... ");
    let rx = solve_brute_force(board, true);
    let mut solution: Option<GameBoard> = None;
    let mut dt: Option<Duration> = None;
    let mut n = 0;
    let time = SystemTime::now();

    if let Ok(board) = rx.recv_timeout(TIMEOUT) {
        dt = Some(time.elapsed().unwrap());
        solution = Some(board);
        n += 1;
    }

    while let Ok(_) = rx.recv_timeout(TIMEOUT) {
        n += 1;
    }

    let dt_total = time.elapsed().unwrap();
    let dt_mean: Duration = if n > 0 {
        dt_total / n
    } else {
        Duration::from_secs(0)
    };
    let dt = dt.unwrap_or(dt_total);
    match solution {
        // DEV-NOTE: use 'ref' to borrow
        Some(ref board) => {
            println!("found {n} solutions.");
            println!("Time for 1st solution:      {dt:.2?}");
            println!("Average time per solution:  {dt_mean:.2?}");
            println!("Total time:                 {dt_total:.2?}");
            println!("\nSolution 1:\n{}\n", board.pretty());
        }
        None => {
            println!("\x1b[91mno solution found!\x1b[0m\n");
        }
    }

    solution
}
