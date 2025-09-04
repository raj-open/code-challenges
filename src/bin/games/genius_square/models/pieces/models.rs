/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::Mul;

use crate::models::arrays::models::BinArray;
use crate::models::constants::board::*;
use crate::models::constants::enums::*;

/// ----------------------------------------------------------------
/// STRUCTS
/// ----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Piece {
    kind: EnumPiece,
    positions: BinArray,
}

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl Piece {
    pub fn from_kind(kind: &EnumPiece, positions: Option<BinArray>) -> Self {
        let kind = kind.clone();
        let positions = positions.unwrap_or_else(|| kind.get_positions());
        Self {kind, positions}
    }

    pub fn from_coords(
        coords: Vec<(usize, usize)>,
        option_kind: Option<EnumPiece>,
    ) -> Self {
        let m = GRID_HEIGHT;
        let n = GRID_WIDTH;
        let positions = BinArray::from_coords(coords, m, n);
        let kind = option_kind.unwrap_or(EnumPiece::Blank);
        Self {kind, positions}
    }

    pub fn to_coords(&self) -> Vec<(usize, usize)> {
        self.positions.to_coords()
    }

    pub fn get_kind(&self) -> EnumPiece {
        self.kind.clone()
    }

    pub fn get_symb(&self) -> String {
        self.kind.as_str().to_string()
    }

    pub fn get_symb_fmt(&self) -> String {
        self.kind.to_formatted().to_string()
    }

    pub fn get_positions(&self) -> &BinArray {
        &self.positions
    }

    #[allow(unused)]
    pub fn get_weight(&self) -> isize {
        self.positions.get_weight()
    }

    #[allow(unused)]
    pub fn get_coweight(&self) -> isize {
        self.positions.get_coweight()
    }

    pub fn to_string(&self) -> String {
        let n = GRID_WIDTH;
        let hbar = "\u{2500}".repeat(n + 2);
        let top = format!("\u{250C}{hbar}\u{2510}");
        let bot = format!("\u{2514}{hbar}\u{2518}");
        let middle = self.positions.get_values().rows()
            .into_iter()
            .map(|row| {
                let line = row
                    .iter()
                    .map(|&val| {
                        if val == 1 {
                            "+".to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("");
                return format!("\u{2502} {line} \u{2502}");
            })
            .collect::<Vec<String>>()
            .join("\n");
        let text = format!("{top}\n{middle}\n{bot}");
        return text;
    }

    #[allow(unused)]
    pub fn transform_hflip(&self, recentre: bool) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_hflip(recentre);
        let result = Self{kind, positions};
        return result;
    }

    #[allow(unused)]
    pub fn transform_vflip(&self, recentre: bool) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_vflip(recentre);
        let result = Self {kind, positions};
        return result;

    }

    #[allow(unused)]
    pub fn transform_transpose(&self, recentre: bool) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_transpose(recentre);
        let result = Self {kind, positions};
        return result;
    }

    #[allow(unused)]
    pub fn transform_rotate(&self, k: i8, recentre: bool) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_rotate(k, recentre);
        let result = Self {kind, positions};
        return result;
    }

    #[allow(unused)]
    pub fn transform_shift(
        &self,
        di: isize,
        dj: isize,
    ) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_shift(di, dj);
        let result = Self {kind, positions};
        return result;
    }

    #[allow(unused)]
    pub fn transform_dither(&self) -> Self {
        let kind = self.get_kind();
        let positions = self.positions.transform_dither();
        let result = Self {kind, positions};
        return result;
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Add for Piece {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let kind = self.get_kind();
        let positions = self.get_positions().to_owned() + other.get_positions().to_owned();
        return Self {kind, positions};
    }
}

impl Mul for Piece {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let kind = self.get_kind();
        let positions = self.get_positions().to_owned() * other.get_positions().to_owned();
        return Self {kind, positions};
    }
}
