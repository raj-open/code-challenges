/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

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
) -> GameBoard {
    let obst = board.get_block().to_owned();
    match recursion(board, &obst, None, None) {
        Some(board_) => {
            return board_;
        },
        None => {
            return board.to_owned();
        }
    }
}

/// ----------------------------------------------------------------
/// AUXILIARY METHODS
/// ----------------------------------------------------------------

fn recursion(
    board: &GameBoard,
    obst: &Piece,
    option_kinds: Option<&[EnumPiece]>,
    option_pbar: Option<&ProgressBar>,
) -> Option<GameBoard> {
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
        if obst.get_coweight() == 0 {
            pbar.finish_and_clear();
            println!("Completed in {:.2?}", pbar.elapsed());
            return Some(board.to_owned());
        }
    } else {
        // otherwise go through all permissible moves for next piece and then proceed recursively
        let kind = &kinds[0].clone();
        let kinds = &kinds[1..];
        let piece0 = Piece::from_kind(kind, None); // initialised piece
        for piece in board.get_configurations(&piece0, &obst) {
            pbar.inc(1);
            // update the obstacle
            let obst_ = obst.clone() + piece.clone();

            // update the solution
            let mut board_ = board.clone();
            board_.add_piece(&kind.clone(), &piece);

            // compute remainder of solution recursively
            match recursion(&mut board_, &obst_, Some(kinds), Some(&pbar)) {
                Some(board_) => {
                    return Some(board_);
                },
                None => {
                    let k = pbar.position();
                    pbar.set_position((k - 1).max(0));
                },
            }
        }
    }

    return None;
}
