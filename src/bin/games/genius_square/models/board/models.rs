/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use ndarray::Array2;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::collections::HashMap;

use crate::models::constants::board::*;
use crate::models::constants::dice::*;
use crate::models::constants::enums::*;
use crate::models::pieces::models::*;

/// ----------------------------------------------------------------
/// STRUCTS
/// ----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct GameBoard {
    block: Piece,
    pieces: HashMap<EnumPiece, Piece>,
    // for dynamic computations
    obstacle_basic: Piece,
    obstacle_dithered: Piece,
}

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl GameBoard {
    pub fn new(block: &Piece) -> Self {
        let pieces: HashMap<EnumPiece, Piece> = HashMap::new();
        let block = block.clone();
        let obstacle_basic = block.clone();
        let obstacle_dithered = block.clone();
        return Self {block, obstacle_basic, obstacle_dithered,  pieces}
    }

    #[allow(unused)]
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

    pub fn get_obstacle(&self) -> &Piece {
        &self.obstacle_basic
    }

    #[allow(unused)]
    pub fn get_obstacle_weight(&self) -> isize {
        self.obstacle_basic.get_weight()
    }

    pub fn get_obstacle_coweight(&self) -> isize {
        self.obstacle_basic.get_coweight()
    }

    pub fn initialise_obstacle(&mut self) {
        self.obstacle_basic = self.block.to_owned();
        self.obstacle_dithered = self.block.to_owned();
    }

    pub fn update_obstacle(&mut self, piece: &Piece) {
        let symb = piece.get_kind();
        self.obstacle_basic += piece.to_owned();
        if NON_ADJACENT.contains(&symb) {
            self.obstacle_dithered += piece.transform_dither().to_owned();
        } else {
            self.obstacle_dithered += piece.to_owned();
        }
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
        let _m = GRID_HEIGHT;
        let n = GRID_WIDTH;
        let field = self.to_array_of_strings(true);

        fn create_border(
            lcorner1: &str,
            fill1: &str,
            lcorner2: &str,
            fill2: &str,
            mid2: &str,
            rcorner: &str,
            n: usize,
        ) -> String {
            let middle = format!("{fill2}{mid2}{fill2}{fill2}").repeat(n-1);
            format!("{lcorner1}{fill1}{fill1}{fill1}{lcorner2}{fill2}{fill2}{middle}{fill2}{rcorner}")
        }

        let top1 = create_border("\u{02554}", "\u{2550}", "\u{02566}", "\u{2550}", "\u{2564}", "\u{2555}", n);
        let top2 = create_border("\u{02560}", "\u{2550}", "\u{0256C}", "\u{2550}", "\u{256A}", "\u{2561}", n);
        let mid = create_border("\u{02560}", "\u{2500}", "\u{0256C}", "\u{2500}", "\u{253C}", "\u{2524}", n);
        let bot = create_border("\u{02559}", "\u{2500}", "\u{02568}", "\u{2500}", "\u{02534}", "\u{02518}", n);

        let head = FACE1_FMT.join(" \u{2502} ").to_string();
        let head = format!("{top1}\n\u{02551}   \u{02551} {head} \u{2502}\n{top2}");

        let middle = field.rows()
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let index = FACE2_FMT[i];
                let line = row.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" \u{2502} ");
                return format!("\u{02551} {index} \u{02551} {line} \u{2502}");
            })
            .collect::<Vec<String>>()
            .join(format!("\n{mid}\n").as_str());

        let text = format!("{head}\n{middle}\n{bot}");

        return text;
    }

    fn to_array_of_strings(&self, formatted: bool) -> Array2<String> {
        let m = GRID_HEIGHT;
        let n = GRID_WIDTH;
        let mut trace: Array2<String> = Array2::from_elem((m, n), " ".to_string());
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
    pub fn get_configurations(
        &self,
        piece: &Piece,
    ) -> impl Iterator<Item = Piece> {
        let mut used: Vec<String> = vec![];
        let obst_positions = self.get_obstacle().get_positions();
        let it = piece
            // convert to positions
            .get_positions()
            // get all possible orientations + shifts which do not collide with obstacle
            .get_configurations(Some(obst_positions))
            // convert to piece
            .map(|pos| {
                let kind = piece.get_kind();
                Piece::from_kind(&kind, Some(pos))
            })
            // skip duplicates
            .filter(move |p| {
                let value = p.to_string();
                let dupl = used.contains(&value);
                used.push(value);
                return !dupl;
            });
        return it;
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}
