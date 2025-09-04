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
}

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl GameBoard {
    pub fn new(block: &Piece) -> Self {
        let pieces: HashMap<EnumPiece, Piece> = HashMap::new();
        return Self {block: block.clone(), pieces}
    }

    pub fn add_piece(&mut self, symb: &EnumPiece, piece: &Piece) {
        self.pieces.insert(symb.clone(), piece.clone());
    }

    pub fn get_block(&self) -> &Piece {
        &self.block
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

        fn create_border(lcorder: &str, fill: &str, mid: &str, rcorner: &str, n: usize) -> String {
            let middle = format!("{fill}{mid}{fill}{fill}").repeat(n);
            format!("{lcorder}{fill}{fill}{middle}{fill}{rcorner}")
        }

        let top1 = create_border("\u{2552}", "\u{2550}", "\u{2564}", "\u{2555}", n);
        let top2 = create_border("\u{255E}", "\u{2550}", "\u{256A}", "\u{2561}", n);
        let mid = create_border("\u{251C}", "\u{2500}", "\u{253C}", "\u{2524}", n);
        let bot = create_border("\u{2558}", "\u{2550}", "\u{2567}", "\u{255B}", n);

        let head = FACE1_FMT.join(" \u{2502} ").to_string();
        let head = format!("{top1}\n\u{2502}   \u{2502} {head} \u{2502}\n{top2}");

        let middle = field.rows()
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let index = FACE2_FMT[i];
                let line = row.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" \u{2502} ");
                return format!("\u{2502} {index} \u{2502} {line} \u{2502}");
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
        let piece = &self.block;
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

    pub fn moves(
        &self,
        piece: &Piece,
        obst: &Piece,
    ) -> impl Iterator<Item = Piece> {
        let mut used: Vec<String> = vec![];
        let it = piece
            // convert to positions
            .get_positions()
            // get all possible moves
            .moves()
            // skip all moves which collide with obstacle
            .filter(|pos| {
                let pos_obst = obst.get_positions();
                let collision = pos.to_owned() * pos_obst.to_owned();
                let penalty = collision.get_weight();
                return penalty == 0;
            })
            // skip all moves which lead to forbidden adjacent pieces
            .filter(|pos| {
                // only need to check for collisions of pieces of a paritcular kind
                if !(NON_ADJACENT.contains(&piece.get_kind())) {
                    return true;
                }
                let pos_dither = pos.transform_dither();
                for (s, q) in self.pieces.iter() {
                    // only need to check for collisions of pieces of a paritcular kind
                    if !(NON_ADJACENT.contains(s)) {
                        continue;
                    }
                    if *s == piece.get_kind() {
                        continue;
                    }
                    let collision = pos_dither.to_owned() * q.get_positions().to_owned();
                    let penalty = collision.get_weight();
                    if penalty > 0 {
                        return false
                    }
                }
                return true;
            })
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
