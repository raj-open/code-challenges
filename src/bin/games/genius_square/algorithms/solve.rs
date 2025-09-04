/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::collections::HashMap;

use crate::models::constants::enums::ENUM_PIECES;
use crate::models::constants::enums::EnumPiece;
use crate::models::pieces::models::Piece;
use crate::models::board::models::GameBoard;

/// ----------------------------------------------------------------
/// METHODS
/// ----------------------------------------------------------------

/// Recursively solves by check all possibilities
pub fn solve_brute_force(
    board: &mut GameBoard,
) {
    let solution = recursion(board, None, None, None);
    let pieces = solution.unwrap_or_else(|| HashMap::new());
    for (kind, piece) in pieces.iter() {
        board.add_piece(&kind, &piece);
    }
}

/// ----------------------------------------------------------------
/// AUXILIARY METHODS
/// ----------------------------------------------------------------

fn recursion(
    board: &GameBoard,
    obst: Option<Piece>,
    kinds: Option<&[EnumPiece]>,
    solution: Option<HashMap<EnumPiece, Piece>>,
) -> Option<HashMap<EnumPiece, Piece>> {

    // if nothing left to solve, then return pieces, provide everything is filled
    let kinds = kinds.unwrap_or(ENUM_PIECES);
    let obst = obst.unwrap_or_else(|| board.get_block().to_owned());
    let pieces = solution.clone().unwrap_or_else(|| HashMap::new());
    if kinds.len() == 0 {
        if obst.get_coweight() == 0 {
            return Some(pieces);
        }
        return None;
    }

    // otherwise go through all permissible moves for next piece and then proceed recursively
    let kind = &kinds[0].clone();
    let kinds_ = &kinds[1..];
    let piece0 = Piece::from_kind(kind, None); // initialised piece
    for piece in board.get_configurations(&piece0, &obst) {
        // update the obstacle
        let obst_ = obst.clone() + piece.clone();

        // update the solution
        let mut pieces_ = pieces.clone();
        pieces_.insert(kind.clone(), piece);

        // compute remainder of solution recursively
        match recursion(board, Some(obst_), Some(kinds_), Some(pieces_)) {
            Some(pieces) => {
                return Some(pieces);
            },
            None => {},
        }
    }
    return None;
}
