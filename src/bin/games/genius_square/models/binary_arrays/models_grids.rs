use itertools::Itertools;
// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use ndarray::Array2;
use ndarray::s as slice;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::AddAssign;

use super::models_arrays::BinArray;

// ----------------------------------------------------------------
// STRUCTS
// ----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Boundary {
    prev: i8,
    next: i8,
}

#[derive(Clone, Debug)]
pub struct Boundaries {
    h: Boundary,
    v: Boundary,
}

#[derive(Clone, Debug)]
pub struct Node {
    label: Option<String>,
    bd: Boundaries,
}

#[derive(Clone, Debug)]
/// A binary grid encases a elements of a binary array with boxes
pub struct BinGrid {
    m: usize,
    n: usize,
    clusters: Vec<Array2<Node>>,
}

// ----------------------------------------------------------------
// IMPLEMENTATIONS
// ----------------------------------------------------------------

impl Boundary {
    pub fn new(prev: i8, next: i8) -> Self {
        Self { prev, next }
    }

    pub fn empty() -> Self {
        Self::new(0, 0)
    }
}

impl Boundaries {
    pub fn new(left: i8, right: i8, top: i8, bot: i8) -> Self {
        let h = Boundary::new(left, right);
        let v = Boundary::new(bot, top);
        Self { h, v }
    }

    pub fn empty() -> Self {
        let h = Boundary::empty();
        let v = Boundary::empty();
        Self { h, v }
    }
}

impl Node {
    pub fn new(label: Option<String>, bd: &Boundaries) -> Self {
        let bd = bd.clone();
        Self { label, bd }
    }

    pub fn empty() -> Self {
        Node::new(None, &Boundaries::empty())
    }

    pub fn from_label(label: &String) -> Self {
        Node::new(Some(label.clone()), &Boundaries::new(-1, 1, -1, 1))
    }

    pub fn display_in_grid(&self) -> Array2<String> {
        let mut field = Array2::from_elem((3, 5), " ".to_string());
        field[(1, 2)] = self.label.clone().map_or_else(|| " ".to_string(), |x| x);

        let bd = self.bd.clone();
        let has_top_boundary = bd.v.prev != 0;
        let has_bot_boundary = bd.v.next != 0;
        let has_left_boundary = bd.h.prev != 0;
        let has_right_boundary = bd.h.next != 0;

        if has_top_boundary {
            field[(0, 0)] = "\u{0253C}".to_string();
            field[(0, 1)] = "\u{02500}".to_string();
            field[(0, 2)] = "\u{02500}".to_string();
            field[(0, 3)] = "\u{02500}".to_string();
            field[(0, 4)] = "\u{0253C}".to_string();
        }

        if has_bot_boundary {
            field[(2, 0)] = "\u{0253C}".to_string();
            field[(2, 1)] = "\u{02500}".to_string();
            field[(2, 2)] = "\u{02500}".to_string();
            field[(2, 3)] = "\u{02500}".to_string();
            field[(2, 4)] = "\u{0253C}".to_string();
        }

        if has_left_boundary {
            field[(0, 0)] = "\u{0253C}".to_string();
            field[(1, 0)] = "\u{02502}".to_string();
            field[(2, 0)] = "\u{0253C}".to_string();
        }

        if has_right_boundary {
            field[(0, 4)] = "\u{0253C}".to_string();
            field[(1, 4)] = "\u{02502}".to_string();
            field[(2, 4)] = "\u{0253C}".to_string();
        }

        return field;
    }
}

impl BinGrid {
    pub fn new(m: usize, n: usize) -> Self {
        let clusters = vec![];
        Self { m, n, clusters }
    }

    #[allow(unused)]
    pub fn get_shape(&self) -> (usize, usize) {
        (self.m, self.n)
    }

    pub fn get_edge_orientation_top(cluster: &Array2<Node>) -> Array2<i8> {
        cluster.mapv(|x| x.bd.v.prev)
    }

    pub fn get_edge_orientation_bottom(cluster: &Array2<Node>) -> Array2<i8> {
        cluster.mapv(|x| x.bd.v.next)
    }

    pub fn get_edge_orientation_left(cluster: &Array2<Node>) -> Array2<i8> {
        cluster.mapv(|x| x.bd.h.prev)
    }

    pub fn get_edge_orientation_right(cluster: &Array2<Node>) -> Array2<i8> {
        cluster.mapv(|x| x.bd.h.next)
    }

    pub fn from_coord(label: &String, i: usize, j: usize, m: usize, n: usize) -> Self {
        let object = BinArray::from_coords(vec![(i, j)], m, n);
        return Self::from_array(label, &object);
    }

    pub fn from_array(label: &String, object: &BinArray) -> Self {
        let (m, n) = object.get_shape();
        let mut cluster = Array2::from_elem((m, n), Node::empty());
        let mut result = Self::new(m, n);
        object.get_values().indexed_iter().for_each(|((i, j), &v)| {
            if v != 0 {
                cluster[(i, j)] = Node::from_label(label);
            }
        });

        // coalesce edges
        let edges_top = Self::get_edge_orientation_top(&cluster);
        let edges_bot = Self::get_edge_orientation_bottom(&cluster);
        let edges_left = Self::get_edge_orientation_left(&cluster);
        let edges_right = Self::get_edge_orientation_right(&cluster);

        let mut coallesce_top = edges_top.clone();
        let mut coallesce_bot = edges_bot.clone();
        let curr = edges_top.slice(slice![1.., ..]);
        let other = edges_bot.slice(slice![..-1, ..]);
        let sum = &curr + &other;
        coallesce_top.slice_mut(slice![1.., ..]).assign(&sum);
        coallesce_bot.slice_mut(slice![..-1, ..]).assign(&sum);

        let mut coallesce_left = edges_left.clone();
        let mut coallesce_right = edges_right.clone();
        let curr = edges_left.slice(slice![.., 1..]);
        let other = edges_right.slice(slice![.., ..-1]);
        let sum = &curr + &other;
        coallesce_left.slice_mut(slice![.., 1..]).assign(&sum);
        coallesce_right.slice_mut(slice![.., ..-1]).assign(&sum);

        coallesce_top.indexed_iter().for_each(|((i, j), &v)| {
            cluster[(i, j)].bd.v.prev = v;
        });

        coallesce_bot.indexed_iter().for_each(|((i, j), &v)| {
            cluster[(i, j)].bd.v.next = v;
        });

        coallesce_left.indexed_iter().for_each(|((i, j), &v)| {
            cluster[(i, j)].bd.h.prev = v;
        });

        coallesce_right.indexed_iter().for_each(|((i, j), &v)| {
            cluster[(i, j)].bd.h.next = v;
        });

        result.clusters = vec![cluster];

        return result;
    }

    pub fn get_node(&self, i: usize, j: usize) -> Node {
        let mut bd = Boundaries::empty();
        let mut label: Option<String> = None;
        for cluster in self.clusters.iter() {
            let node_ = cluster[(i, j)].clone();
            let label_ = node_.label;
            let bd_ = node_.bd;
            if let Some(x) = label_ {
                label = Some(x);
            }
            if bd.v.next == 0 {
                bd.v.next = bd_.v.next;
            }
            if bd.v.prev == 0 {
                bd.v.prev = bd_.v.prev;
            }
            if bd.h.next == 0 {
                bd.h.next = bd_.h.next;
            }
            if bd.h.prev == 0 {
                bd.h.prev = bd_.h.prev;
            }
        }
        let node = Node::new(label, &bd);
        return node;
    }
}

impl Display for BinGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (m, n) = self.get_shape();

        // compute fields of characters
        let fields: Vec<Array2<String>> = (0..m)
            .map(|i| {
                // compute fields of characters
                let fields: Vec<Array2<String>> = (0..n)
                    // get array of chars representing part of grid
                    .map(|j| self.get_node(i, j).display_in_grid())
                    .collect();

                // determine size of merge
                let m = fields.iter().map(|field_| field_.nrows()).max().unwrap_or(0);
                let n = 1 + fields.iter().map(|field_| field_.ncols() - 1).sum::<usize>();
                let mut field = Array2::from_elem((m, n), " ".to_string());

                // h-join chars in grid, taking care of boundaries
                let mut j0 = 0;
                for (k, field_) in fields.iter().enumerate() {
                    let m_ = field_.nrows();
                    let n_ = field_.ncols();
                    let j1 = if k > 0 && n_ > 0 {
                        j0 + n_ - 1
                    } else {
                        j0 + n_
                    };
                    let mut view = field.slice_mut(slice![..m_, j0..j1]);
                    if k == 0 {
                        view.assign(&field_);
                    } else {
                        view.assign(&field_.slice(slice![.., 1..]));
                    }
                    // on boundary allow empty values to be overwritten
                    if j0 > 0 {
                        for i in 0..m_ {
                            if field[(i, j0 - 1)].trim() == "" {
                                field[(i, j0 - 1)] = field_[(i, 0)].clone();
                            }
                        }
                    }
                    j0 = j1;
                }

                return field;
            })
            .collect();

        // determine size of merge
        let m = 1 + fields.iter().map(|field_| field_.nrows() - 1).sum::<usize>();
        let n = fields.iter().map(|field_| field_.ncols()).max().unwrap_or(0);
        let mut field = Array2::from_elem((m, n), " ".to_string());

        // v-join chars in grid, taking care of boundaries
        let mut i0 = 0;
        for (k, field_) in fields.iter().enumerate() {
            let m_ = field_.nrows();
            let n_ = field_.ncols();
            let i1 = if k > 0 && m_ > 0 {
                i0 + m_ - 1
            } else {
                i0 + m_
            };
            let mut view = field.slice_mut(slice![i0..i1, ..n_]);
            if k == 0 {
                view.assign(&field_);
            } else {
                view.assign(&field_.slice(slice![1.., ..]));
            }
            // on boundary allow empty values to be overwritten
            if i0 > 0 {
                for j in 0..n_ {
                    if field[(i0 - 1, j)].trim() == "" {
                        field[(i0 - 1, j)] = field_[(0, j)].clone();
                    }
                }
            }
            i0 = i1;
        }

        // finally, join field chars:
        let text = field.rows().into_iter().map(|row| row.iter().join("")).join("\n");
        write!(f, "{}", text)
    }
}

impl AddAssign for BinGrid {
    fn add_assign(&mut self, other: Self) {
        self.clusters.extend(other.clusters.clone());
    }
}

impl Add for BinGrid {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        result += other;
        return result;
    }
}
