// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use std::fmt::Debug;
use std::string::ToString;

use crate::models::constants::FACE1;
use crate::models::constants::FACE2;

// ----------------------------------------------------------------
// STRUCTS
// ----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Die {
    i: usize,
    j: usize,
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS
// ----------------------------------------------------------------

impl Die {
    pub fn from_string(face: &String) -> Die {
        let chars: Vec<String> = face.chars().map(|c| c.to_string()).collect();
        let char1 = chars.first().unwrap();
        let char2 = chars.get(1).unwrap();
        let index1 = FACE1.iter().position(|x| x == char1).unwrap();
        let index2 = FACE2.iter().position(|x| x == char2).unwrap();
        Die { i: index2, j: index1 }
    }

    #[allow(unused)]
    pub fn from_coords(i: usize, j: usize) -> Die {
        Die { i, j }
    }

    #[allow(unused)]
    pub fn to_coords(&self) -> (usize, usize) {
        (self.i, self.j)
    }
}

impl ToString for Die {
    fn to_string(&self) -> String {
        let char1: String = FACE1[self.j].to_string();
        let char2: String = FACE2[self.i].to_string();
        format!("{char1}{char2}")
    }
}
