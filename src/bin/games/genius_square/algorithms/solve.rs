/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

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
    match recursion(board, &obst, None) {
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
    kinds: Option<&[EnumPiece]>,
) -> Option<GameBoard> {
    let kinds = kinds.unwrap_or(ENUM_PIECES);

    if kinds.len() == 0 {
        // if nothing left to solve, then return pieces, provide everything is filled
        if obst.get_coweight() == 0 {
            return Some(board.to_owned());
        }
    } else {
        // otherwise go through all permissible moves for next piece and then proceed recursively
        let kind = &kinds[0].clone();
        let kinds = &kinds[1..];
        let piece0 = Piece::from_kind(kind, None); // initialised piece
        for piece in board.get_configurations(&piece0, &obst) {
            // update the obstacle
            let obst_ = obst.clone() + piece.clone();

            // update the solution
            let mut board_ = board.clone();
            board_.add_piece(&kind.clone(), &piece);

            // compute remainder of solution recursively
            match recursion(&mut board_, &obst_, Some(kinds)) {
                Some(board_) => {
                    return Some(board_);
                },
                None => {},
            }
        }
    }

    return None;
}
