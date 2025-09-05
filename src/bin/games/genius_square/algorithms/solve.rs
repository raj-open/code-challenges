/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use itertools::Itertools;
use std::time::Duration;
use std::thread::spawn;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use crate::models::constants::enums::ENUM_PIECES;
use crate::models::constants::enums::EnumPiece;
use crate::models::pieces::models::Piece;
use crate::models::board::models::GameBoard;

/// ----------------------------------------------------------------
/// TYPES
/// ----------------------------------------------------------------

type CHAN = (Duration, GameBoard);

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

/// Recursively solves by check all possibilities
pub fn solve_brute_force(
    board: &GameBoard,
) -> Receiver<CHAN> {
    let (tx, rx) = channel::<CHAN>();
    let mut board = board.clone();
    board.initialise_obstacle();
    // DEV-NOTE: This is necessary to ensure that no locking occurs.
    spawn(move || {
        recursion(&tx, &board, None, None);
    });
    return rx;
}

/// ----------------------------------------------------------------
/// AUXILIARY METHODS
/// ----------------------------------------------------------------

fn recursion(
    tx: &Sender<CHAN>,
    board: &GameBoard,
    option_kinds: Option<&[EnumPiece]>,
    option_pbar: Option<&ProgressBar>,
) {
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
            let message = (dt, board.to_owned());
            tx.send(message).unwrap();
        }
    } else {
        // find the next piece which has the fewest number of next possible moves
        let kinds: Vec<EnumPiece> = kinds.iter()
            .map(|kind| {
                let piece = Piece::from_kind(kind, None);
                let iterator = board.get_configurations(&piece);
                let n = iterator.count();
                return (kind, n);
            })
            // sort by ascending values of size of possibilities
            .sorted_by_key(|&(_, n)| n as isize)
            .map(|(kind, _)| kind.clone())
            .collect();

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
            recursion(tx, &board_, Some(kinds), Some(&pbar));
            let k = pbar.position();
            pbar.set_position((k - 1).max(0));
        }
    }
}
