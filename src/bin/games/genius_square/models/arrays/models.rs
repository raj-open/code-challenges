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

    /// Gets the list of co-ordinates of the entries which are non-zero
    pub fn to_coords(&self) -> Vec<(usize, usize)> {
        self.values
            .indexed_iter()
            .filter_map(|((i, j), &v)| if v == 0 { None } else { Some((i, j)) })
            .collect()
    }

    /// Determines a single co-ordinate to be used as an anchor point.
    /// If none exists, defaults to (0, 0).
    pub fn get_anchor(&self) -> (usize, usize) {
        self.values
            .indexed_iter()
            .find_or_first(|&(_, &v)| v != 0)
            .map_or((0, 0), |((i, j), _)| (i, j))
    }

    pub fn get_weight(&self) -> isize {
        self.values
            .mapv(|x| if x == 0 {0} else {1})
            .sum()
    }

    pub fn get_coweight(&self) -> isize {
        self.transform_invert().get_weight()
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

    /// Flips 0s and 1s
    pub fn transform_invert(&self) -> Self {
        let m = self.m;
        let n = self.n;
        let values = self.values.mapv(|x| if x == 0 {1} else {0});
        return Self {m, n, values};
    }

    pub fn transform_shift(
        &self,
        di: isize,
        dj: isize,
    ) -> Self {
        // create blank 3 x 3 meta block
        let m = self.m;
        let n = self.n;
        let mut slate = Array2::<u8>::zeros((3*m, 3*n));

        // slot in values in location shifted from the middle
        let i0 = self.m as isize + di;
        let i1 = (self.m as isize) + i0;
        let j0 = self.n as isize + dj;
        let j1 = (self.n as isize) + j0;
        let mut view = slate.slice_mut(slice![i0..i1, j0..j1]);
        view.assign(&self.values);

        // restrict to "middle" part
        let i0 = self.m;
        let i1 = self.m + i0;
        let j0 = self.n;
        let j1 = self.n + j0;
        let values = slate.slice_mut(slice![i0..i1, j0..j1]).to_owned();
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

    pub fn transform_rotate(&self, k: i8, recentre: bool) -> Self {
        match k {
            1 => {
                return self.transform_transpose(false).transform_vflip(recentre);
            },
            -1 => {
                return self.transform_vflip(false).transform_transpose(recentre);
            },
            _ => {
                return self.clone();
            }
        }
    }

    /// For collision comparison
    pub fn transform_dither(&self) -> Self {
        let (m, n) = self.get_shape();
        let coords = self.to_coords();
        let mut arr = BinArray::from_coords(coords, m + 2, n + 2);
        arr = arr.transform_shift(1, 1);
        let arr1 = arr.transform_shift(-1, 0);
        let arr2 = arr.transform_shift(1, 0);
        let arr3 = arr.transform_shift(0, -1);
        let arr4 = arr.transform_shift(0, 1);
        arr = arr + arr1 + arr2 + arr3 + arr4;
        let values = arr.values
            .slice(slice![1..-1, 1..-1])
            .to_owned();
        let result = Self {m, n, values};
        return result;
    }

    /// Determines all possible configurations
    /// of the same array subject to
    ///
    /// - rotations,
    /// - v- and h-flips,
    /// - v- and h-shifts
    ///
    /// provided the moves preserve the "weight" of the shadow in the array
    pub fn get_configurations(&self) -> impl Iterator<Item = Self> {
        let m = self.m as isize;
        let n = self.n as isize;
        let iterator = iproduct!(
            [0, 1, -1],
            [false, true],
            [false, true],
            (0.. m),
            (0.. n),
        )
            .map(|(rot, vflip, hflip, di, dj)| {
                // recover original
                let mut arr = self.clone();
                if rot != 0 {
                    arr = arr.transform_rotate(rot, false);
                }
                if vflip {
                    arr = arr.transform_vflip(false);
                }
                if hflip {
                    arr = arr.transform_hflip(false);
                }
                if hflip | vflip | (rot != 0) {
                    arr = arr.recentre();
                }
                arr = arr.transform_shift(di, dj);
                return arr;
            })
            // if geometric operations shift shape off the grid, skip
            .filter(|arr| {
                let wt = self.get_weight();
                return arr.get_weight() >= wt;
            });
        return iterator;
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
        let mut values = self.values.to_owned() + other.values.to_owned();
        values = values.mapv(|x| x.min(1));
        return Self {m, n, values};
    }
}

impl Mul for BinArray {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let m = self.m;
        let n = self.n;
        let values = self.values.to_owned() * other.values.to_owned();
        return Self {m, n, values};
    }
}
