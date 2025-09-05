/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::models::constants::dice::*;

/// ----------------------------------------------------------------
/// STRUCTS
/// ----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Die {
    i: usize,
    j: usize,
}

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl Die {
    pub fn from_string(face: &String) -> Die {
        let chars: Vec<String> = face.chars().map(|c| c.to_string()).collect();
        let char1 = chars.get(0).unwrap();
        let char2 = chars.get(1).unwrap();
        let index1 = FACE1.iter().position(|x| x == char1).unwrap();
        let index2 = FACE2.iter().position(|x| x == char2).unwrap();
        return Die {
            i: index2,
            j: index1,
        }
    }

    pub fn to_string(&self) -> String {
        let char1: String = FACE1[self.j].to_string();
        let char2: String = FACE2[self.i].to_string();
        return format!("{char1}{char2}");
    }

    #[allow(unused)]
    pub fn from_coords(i: usize, j: usize) -> Die {
        return Die {i, j}
    }

    #[allow(unused)]
    pub fn to_coords(&self) -> (usize, usize) {
        return (self.i, self.j)
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_string())
    }
}
