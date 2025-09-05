/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::time::Duration;

use crate::models::constants::enums::ENUM_PIECES;
use crate::models::constants::enums::EnumPiece;
use crate::models::pieces::models::Piece;
use crate::models::board::models::GameBoard;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

/// Recursively solves by check all possibilities
pub fn solve_brute_force(
    board: &GameBoard,
) -> (Duration, Option<GameBoard>) {
    let mut board = board.clone();
    board.initialise_obstacle();
    let result = recursion(&board, None, None);
    return result;
}

/// ----------------------------------------------------------------
/// AUXILIARY METHODS
/// ----------------------------------------------------------------

fn recursion(
    board: &GameBoard,
    option_kinds: Option<&[EnumPiece]>,
    option_pbar: Option<&ProgressBar>,
) -> (Duration, Option<GameBoard>) {
    let kinds = option_kinds.unwrap_or(ENUM_PIECES);
    let n = kinds.len() as u64;

    let pbar0 = ProgressBar::new(n);
    let pbar: &ProgressBar;
    match option_pbar {
        Some(pbar_) => {
            pbar = &pbar_;
        },
        None => {
            pbar = &pbar0;
            let style = ProgressStyle::with_template("{spinner:.white} [{elapsed_precise}] [{wide_bar:.white}] {pos}/{len} ({eta_precise})");
            pbar.set_style(style.unwrap())
        }
    }

    if n == 0 {
        // if nothing left to solve, then return pieces, provide everything is filled
        if board.get_obstacle_coweight() == 0 {
            pbar.finish_and_clear();
            let dt = pbar.elapsed();
            return (dt, Some(board.to_owned()));
        }
    } else {
        // otherwise go through all permissible moves for next piece and then proceed recursively
        let kind = &kinds[0].clone();
        let kinds = &kinds[1..];
        let piece0 = Piece::from_kind(kind, None); // initialised piece
        for piece in board.get_configurations(&piece0) {
            pbar.inc(1);
            let mut board_ = board.clone();

            // update the solution
            board_.add_piece(&kind.clone(), &piece);

            // update the obstacle
            board_.update_obstacle(&piece);

            // compute remainder of solution recursively
            let (dt, result) = recursion(&board_, Some(kinds), Some(&pbar));
            match result {
                Some(_) => {
                    return (dt, result);
                },
                None => {
                    let k = pbar.position();
                    pbar.set_position((k - 1).max(0));
                },
            }
        }
    }

    let dt = pbar.elapsed();
    return (dt, None);
}
