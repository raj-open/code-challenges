/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use ndarray::Array2;
use ndarray::s as slice;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::ops::Add;
use std::ops::Mul;
use itertools::iproduct;

/// ----------------------------------------------------------------
/// STRUCTS
/// ----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct BinArray {
    m: usize,
    n: usize,
    values: Array2<u8>,
}

/// ----------------------------------------------------------------
/// IMPLEMENTATIONS
/// ----------------------------------------------------------------

impl BinArray {
    pub fn from_coords(
        coords: Vec<(usize, usize)>,
        m: usize,
        n: usize,
    ) -> Self {
        let mut values = Array2::<u8>::zeros((m, n));
        for coord in coords {
            values[[coord.0, coord.1]] = 1;
        }
        Self {m, n, values}
    }

    pub fn to_coords(&self) -> Vec<(usize, usize)> {
        let mut coords: Vec<(usize, usize)> = vec![];
        for (i, row) in self.values.rows().into_iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    coords.push((i, j));
                }
            }
        }
        return coords;
    }

    pub fn get_weight(&self) -> u8 {
        let values = self.values.mapv(|x| if x == 0 {0} else {1});
        let weight = values.sum();
        return weight;
    }

    pub fn get_coweight(&self) -> u8 {
        let covalues = self.values.mapv(|x| if x == 0 {1} else {0});
        let coweight = covalues.sum();
        return coweight;
    }

    pub fn get_shape(&self) -> (usize, usize) {
        (self.m, self.n)
    }

    pub fn get_values(&self) -> Array2<u8> {
        self.values.clone()
    }

    /// Shifts array as far as possible to top left
    pub fn recentre(&self) -> Self {
        // determine maximal h+v-shifts
        let coords = self.to_coords();
        let i_min: usize = *coords.iter().map(|(i, _)| i).min().unwrap_or(&0);
        let j_min: usize = *coords.iter().map(|(_, j)| j).min().unwrap_or(&0);
        // shift coords
        let result = self.transform_shift(-(i_min as isize), -(j_min as isize));
        return result;
    }

    pub fn transform_shift(
        &self,
        di: isize,
        dj: isize,
    ) -> Self {
        // shift coords
        let m = self.m as isize;
        let n = self.n as isize;
        let coords = self.to_coords();
        let coords: Vec<(usize, usize)> = coords
            .iter()
            .map(|&(i, j)| (i as isize, j as isize))
            .map(|(i, j)| (i + di, j + dj))
            .filter(|&(i, j)| (0 <= i && i < m && 0 <= j && j < n))
            .map(|(i, j)| (usize::try_from(i).unwrap(), usize::try_from(j).unwrap()))
            .collect();

        // update values
        let m = self.m;
        let n = self.n;
        let mut values = Array2::<u8>::zeros((m, n));
        for coord in coords {
            values[[coord.0, coord.1]] = 1;
        }
        let result = Self {m, n, values};
        return result;
    }

    pub fn transform_hflip(&self, recentre: bool) -> Self{
        let m = self.m;
        let n = self.n;
        let values = self.values.slice(slice![.., ..;-1]).to_owned();
        let mut result = Self {m, n, values};
        if recentre {
            result = result.recentre();
        }
        return result;
    }

    pub fn transform_vflip(&self, recentre: bool) -> Self {
        let m = self.m;
        let n = self.n;
        let values = self.values.slice(slice![..;-1, ..]).to_owned();
        let mut result = Self {m, n, values};
        if recentre {
            result = result.recentre();
        }
        return result;

    }

    pub fn transform_transpose(&self, recentre: bool) -> Self {
        let m = self.m;
        let n = self.n;
        let values = self.values.t().to_owned();
        let mut result = Self {m, n, values};
        if recentre {
            result = result.recentre();
        }
        return result;
    }

    /// For collision comparison
    pub fn transform_dither(&self) -> Self {
        let (m, n) = self.get_shape();
        let coords = self.to_coords();
        let arr = BinArray::from_coords(coords, m + 2, n + 2);
        arr.transform_shift(1, 1);
        let arr1 = arr.transform_shift(-1, 0).to_owned();
        let arr2 = arr.transform_shift(1, 0).to_owned();
        let arr3 = arr.transform_shift(0, -1).to_owned();
        let arr4 = arr.transform_shift(0, 1).to_owned();
        let arr = arr + arr1 + arr2 + arr3 + arr4;
        let values = arr.values
            .mapv(|x| x.min(1))
            .slice(slice![1..-1, 1..-1])
            .to_owned();
        let result = Self {m, n, values};
        return result;
    }

    pub fn moves(&self) -> impl Iterator<Item = Self> {
        let m = self.m;
        let n = self.n;
        let range_i: Vec<isize> = (0.. m).map(|i| i as isize).collect();
        let range_j: Vec<isize> = (0.. n).map(|j| j as isize).collect();
        let params = iproduct!(
            [false, true],
            [false, true],
            [false, true],
            range_i,
            range_j,
        );
        let moves = params
            .map(|(hflip, vflip, tr, di, dj)| {
                // recover original
                let mut arr = self.clone();
                if hflip {
                    arr = arr.transform_hflip(false);
                }
                if vflip {
                    arr = arr.transform_vflip(false);
                }
                if tr {
                    arr = arr.transform_transpose(false);
                }
                if hflip | vflip | tr {
                    arr = arr.recentre();
                }
                arr = arr.transform_shift(di, dj);
                return arr;
            })
            // if geometric operations shift shape off the grid, skip
            .filter(|arr| {
                let wt = self.get_weight();
                return arr.get_weight() == wt;
            });
        return moves;
    }
}

impl Display for BinArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.values)
    }
}

impl Add for BinArray {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let m = self.m;
        let n = self.n;
        let values = self.values + other.values;
        return Self {m, n, values};
    }
}

impl Mul for BinArray {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let m = self.m;
        let n = self.n;
        let values = self.values * other.values;
        return Self {m, n, values};
    }
}
