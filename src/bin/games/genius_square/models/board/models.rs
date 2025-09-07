// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use ndarray::Array2;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::collections::HashMap;

use general::_core::strings::join_multiline_strings;

use crate::models::constants::EnumPiece;
use crate::models::constants::FACE1_FMT;
use crate::models::constants::FACE2_FMT;
use crate::models::constants::GRID_HEIGHT;
use crate::models::constants::GRID_WIDTH;
use crate::models::constants::NON_ADJACENT;
use crate::models::pieces::Piece;
use crate::models::binary_arrays::BinGrid;

// ----------------------------------------------------------------
// STRUCTS
// ----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GameBoard {
    block: Piece,
    pieces: HashMap<EnumPiece, Piece>,
    // for dynamic computations
    obstacle_basic: Piece,
    obstacle_dithered: Piece,
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS
// ----------------------------------------------------------------

impl GameBoard {
    pub fn new(block: &Piece) -> Self {
        let pieces: HashMap<EnumPiece, Piece> = HashMap::new();
        let block = block.clone();
        let obstacle_basic = block.clone();
        let obstacle_dithered = block.clone();
        return Self {block, obstacle_basic, obstacle_dithered,  pieces}
    }

    pub fn get_shape(&self) -> (usize, usize) {
        self.block.get_shape()
    }

    pub fn add_piece(&mut self, symb: &EnumPiece, piece: &Piece) {
        self.pieces.insert(symb.clone(), piece.clone());
    }

    #[allow(unused)]
    pub fn set_pieces(&mut self, pieces: &HashMap<EnumPiece, Piece>) {
        self.pieces = pieces.clone();
    }

    pub fn get_block(&self) -> &Piece {
        &self.block
    }

    pub fn initialise_obstacle(&mut self) {
        self.obstacle_basic = self.block.to_owned();
        self.obstacle_dithered = self.block.to_owned();
    }

    pub fn get_obstacle(&self, kind: &EnumPiece) -> &Piece {
        if NON_ADJACENT.contains(&kind) {
            &self.obstacle_dithered
        } else {
            &self.obstacle_basic
        }
    }

    pub fn update_obstacle(&mut self, piece: &Piece) {
        let symb = piece.get_kind();
        self.obstacle_basic += piece.to_owned();
        if NON_ADJACENT.contains(&symb) {
            let piece_dithered = piece.transform_dither();
            self.obstacle_dithered += piece_dithered;
        } else {
            self.obstacle_dithered += piece.to_owned();
        }
    }

    #[allow(unused)]
    pub fn get_obstacle_weight(&self) -> isize {
        self.obstacle_basic.get_weight()
    }

    pub fn get_obstacle_coweight(&self) -> isize {
        self.obstacle_basic.get_coweight()
    }

    pub fn to_string(&self) -> String {
        let field = self.to_array_of_strings(false);
        let text = Self::array_to_string(&field);
        return text;
    }

    #[allow(unused)]
    pub fn to_formatted(&self) -> String {
        let field = self.to_array_of_strings(true);
        let text = Self::array_to_string(&field);
        return text;
    }

    pub fn pretty(&self) -> String {
        let (m, n) = self.get_shape();

        let space = "      ";
        let end1 = format!("{space}\u{02554}\u{02550}\n{space}\u{02551} \n{space}\u{0255A}\u{02550}");
        let end2 = format!("\u{02550}\u{02557}\n \u{02551}\n\u{02550}\u{0255D}");
        let blocks: Vec<String> = FACE1_FMT.iter().map(|&x| format!("\u{02550}\n{x}\n\u{02550}")).collect();
        let sep = format!("\u{02550}\u{02566}\u{02550}\n \u{02551} \n\u{02550}\u{02569}\u{02550}");
        let xlabels = join_multiline_strings(&blocks, Some(&sep), "");
        let xlabels = join_multiline_strings(&[end1, xlabels, end2].to_vec(), None, "");

        let end1 = format!("\u{02554}\u{02550}\u{02550}\u{02550}\u{02557}");
        let end2 = format!("\u{0255A}\u{02550}\u{02550}\u{02550}\u{0255D}");
        let blocks: Vec<String> = FACE2_FMT.iter().map(|&x| format!("\u{02551} {x} \u{02551}")).collect();
        let sep = format!("\n\u{02560}\u{02550}\u{02550}\u{02550}\u{02563}\n");
        let ylabels = blocks.join(&sep);
        let ylabels = [end1, ylabels, end2].join("\n");

        let mut grid = BinGrid::new(m, n);
        let symb = self.block.get_symb_fmt();
        self.block.to_coords().iter().for_each(|&(i, j)| {
            grid += BinGrid::from_coord(&symb, i, j, m, n);
        });

        self.pieces.values().for_each(|piece| {
            let symb = piece.get_symb_fmt();
            let pos = piece.get_positions();
            grid += BinGrid::from_array(&symb, pos);
        });

        let middle = grid.to_string();

        let middle = join_multiline_strings(&[ylabels.to_owned(), middle].to_vec(), None, " ");

        let text = format!("{xlabels}\n{middle}");

        return text;
    }

    fn to_array_of_strings(&self, formatted: bool) -> Array2<String> {
        let m = GRID_HEIGHT;
        let n = GRID_WIDTH;
        let mut trace = Array2::from_elem((m, n), " ".to_string());
        let piece = self.get_block();
        for (i, j) in piece.to_coords() {
            let alpha = if formatted { piece.get_symb_fmt() } else { piece.get_symb() };
            trace[[i, j]] = alpha;
        }
        for (_, piece) in self.pieces.iter() {
            for (i, j) in piece.to_coords() {
                let alpha = if formatted { piece.get_symb_fmt() } else { piece.get_symb() };
                trace[[i, j]] = alpha;
            }
        }
        return trace;
    }

    fn array_to_string(field: &Array2<String>) -> String {
        let n = GRID_WIDTH;
        let hbar = "\u{2500}".repeat(n + 2);
        let top = format!("\u{250C}{hbar}\u{2510}");
        let bot = format!("\u{2514}{hbar}\u{2518}");
        let middle = field.rows()
            .into_iter()
            .map(|row| {
                let line = row.iter().map(|s| s.as_str()).collect::<String>();
                return format!("\u{2502} {line} \u{2502}");
            })
            .collect::<Vec<String>>()
            .join("\n");
        let text = format!("{top}\n{middle}\n{bot}");
        return text;
    }

    /// Determines all possible configurations
    /// of the same piece subject to
    ///
    ///  - rotations,
    /// - v- and h-flips,
    /// - v- and h-shifts
    ///
    /// provided the moves preserve the "weight" of the shadow in the array
    /// and provided
    ///
    /// - no collisions occur with already placed pieces (marked by `obst`)
    /// - the piece is not adjacent to certain other pieces.
    pub fn get_configurations(&self, piece: &Piece) -> impl Iterator<Item = Piece> {
        // get obstacle in depenence on type of piece
        let kind = piece.get_kind();
        let obst = self.get_obstacle(&kind).get_positions();

        // construct iterator
        let it = piece
            // convert to positions
            .get_positions()
            // get all possible orientations + shifts which do not collide with obstacle
            .get_configurations(Some(obst))
            // convert to piece
            .map(move |pos| Piece::from_kind(&kind, Some(pos)));
        return it;
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}
