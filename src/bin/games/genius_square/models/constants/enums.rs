/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::models::arrays::models::BinArray;
use super::board::*;
use super::pieces::*;

/// ----------------------------------------------------------------
/// STRUCTS AND CONSTANTS
/// ----------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnumPiece {
    Block,
    Symb1,
    Symb2,
    Symb3,
    Symb4,
    C,
    L,
    T,
    X,
    Z,
}

pub const ENUM_PIECES: &[EnumPiece] = &[
    EnumPiece::Block,
    EnumPiece::Symb1,
    EnumPiece::Symb2,
    EnumPiece::Symb3,
    EnumPiece::Symb4,
    EnumPiece::C,
    EnumPiece::L,
    EnumPiece::T,
    EnumPiece::X,
    EnumPiece::Z,
];

pub const NON_ADJACENT: &[EnumPiece] = &[
    EnumPiece::Symb1,
    EnumPiece::Symb2,
    EnumPiece::Symb3,
    EnumPiece::C,
];

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl EnumPiece {
    #[allow(unused)]
    pub const fn as_str(&self) -> &'static str {
        match self {
            EnumPiece::Block => SYMB_BLOCK,
            EnumPiece::Symb1 => SYMB_PIECE_1,
            EnumPiece::Symb2 => SYMB_PIECE_2,
            EnumPiece::Symb3 => SYMB_PIECE_3,
            EnumPiece::Symb4 => SYMB_PIECE_4,
            EnumPiece::C => SYMB_PIECE_C,
            EnumPiece::L => SYMB_PIECE_L,
            EnumPiece::T => SYMB_PIECE_T,
            EnumPiece::X => SYMB_PIECE_X,
            EnumPiece::Z => SYMB_PIECE_Z,
        }
    }

    #[allow(unused)]
    pub const fn to_formatted(&self) -> &'static str {
        match self {
            EnumPiece::Block => SYMB_FMT_BLOCK,
            EnumPiece::Symb1 => SYMB_FMT_PIECE_1,
            EnumPiece::Symb2 => SYMB_FMT_PIECE_2,
            EnumPiece::Symb3 => SYMB_FMT_PIECE_3,
            EnumPiece::Symb4 => SYMB_FMT_PIECE_4,
            EnumPiece::C => SYMB_FMT_PIECE_C,
            EnumPiece::L => SYMB_FMT_PIECE_L,
            EnumPiece::T => SYMB_FMT_PIECE_T,
            EnumPiece::X => SYMB_FMT_PIECE_X,
            EnumPiece::Z => SYMB_FMT_PIECE_Z,
        }
    }

    pub fn get_positions(&self) -> BinArray {
        let raw = match self {
            EnumPiece::Block => BLOCK,
            EnumPiece::Symb1 => PIECE_1,
            EnumPiece::Symb2 => PIECE_2,
            EnumPiece::Symb3 => PIECE_3,
            EnumPiece::Symb4 => PIECE_4,
            EnumPiece::C => PIECE_C,
            EnumPiece::L => PIECE_L,
            EnumPiece::T => PIECE_T,
            EnumPiece::X => PIECE_X,
            EnumPiece::Z => PIECE_Z,
        };
        let mut coords: Vec<(usize, usize)> = vec![];
        for (i, line) in raw.lines().enumerate() {
            for (j, a) in line.chars().enumerate() {
                if a.to_string() == "+" {
                    coords.push((i, j));
                }
            }
        }
        let m = GRID_HEIGHT;
        let n = GRID_WIDTH;
        let positions = BinArray::from_coords(coords, m, n);
        return positions;
    }
}

impl Display for EnumPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_formatted())
    }
}
