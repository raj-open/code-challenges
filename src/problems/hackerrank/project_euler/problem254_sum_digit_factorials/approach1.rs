/// # First approach #
///
/// Computes Primes using the Sieve of Eratosthenes.
/// Then efficiently computes the cumulutative sum of primes
/// below certain integers.

/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::io;
use std::io::BufRead;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::Ord;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Stdin;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::str::FromStr;

/// ----------------------------------------------------------------
/// MAIN
/// ----------------------------------------------------------------

/// entry point when used as a script
#[allow(unused)]
fn main() {
    let lines = read_input(&io::stdin());
    let t = lines.iter().nth(0).unwrap().parse::<usize>();
    let queries: Vec<(i64, i64)> = lines
        .iter()
        .skip(1)
        .map(|line| {
            let mut words = line.split_whitespace();
            let n = words.next().unwrap().parse::<i64>().unwrap();
            let modulus = words.next().unwrap().parse::<i64>().unwrap();
            return (n, modulus);
        })
        .collect();

    let results = run(&queries);

    // for (n, modulus) in queries.iter() {
    //     let index = *n as usize;
    //     let value = *results.get(&(*n, *modulus)).unwrap();
    //     println!("{:?}", value);
    // }
}

pub fn run(queries: &Vec<(i64, i64)>) -> HashMap<i64, i64> {
    let mut moduli: Vec<i64> = queries.iter().map(|(_, modulus)| *modulus).collect();
    moduli.sort();
    moduli.dedup();

    // let results: HashMap<(i64, i64), i64> = HashMap::new();
    let numbers: Vec<i64> = queries.iter().map(|(n, _)| *n).collect();

    // compute f (use max-values, as we only need f and s(f) for search purposes)
    let graph_sf = compute_factorial_diagonals_of_cumsums(&numbers);

    // compute g
    let graph_g = invert_graph(graph_sf);

    // // compute s(g)
    // let graph_sg: HashMap<_, _> = graph_g
    //     .into_iter()
    //     .map(|(k, x)| (k, diagonalise(x)))
    //     .collect();

    // queries.iter().for_each(|&(n, modulus)| {
    //     // println!("f: {:?}", graph_sf);
    //     // println!("sf: {:?}", graph_sf.clone());
    //     // println!("g: {:?}", graph_g.clone());
    //     // println!("sg: {:?}", graph_sg);

    //     // store results for n-th values
    //     // for n in numbers.iter() {
    //     //     match graph_sg.get(&(*n as usize)) {
    //     //         Some(x) => {
    //     //             results.insert((*n, modulus), x.value);
    //     //         },
    //     //         None => {
    //     //             panic!("no value found for g({})", n)
    //     //         }
    //     //     }
    //     // }
    // });

    return graph_g;
}

/// ----------------------------------------------------------------
/// HELPER METHODS
/// ----------------------------------------------------------------

/// computes cumsums of factorials
/// and diagonal-sums
/// until all possible values of diagonal sums are found
fn compute_factorial_diagonals_of_cumsums(numbers: &Vec<i64>) -> HashMap<i64, i64> {
    let mut sgraph: HashMap<i64, i64> = HashMap::new();
    let mut values: HashSet<i64> = HashSet::new();
    let mut k: i64 = 0;
    let mut sum: i128 = 0;
    let mut value: i128 = 1;
    let n_max = numbers.iter().fold(0, |prev, n| prev.max(*n));

    fn contains_all_values(values: &HashSet<i64>, n: i64) -> bool {
        (1..(n + 1)).all(|k| values.contains(&(k as i64)))
    }

    while !contains_all_values(&values, n_max) {
        k += 1;
        value *= k as i128;
        sum += value;
        let diag = diagonalise(sum) as i64;
        sgraph.insert(k, diag.clone());
        values.insert(diag.clone());
    }

    return sgraph;
}

/// computes the inverse of a functional graph
/// taking mapping each value to its "first" key
fn invert_graph<K, V>(graph: HashMap<K, V>) -> HashMap<V, K>
where
    K: Clone + Hash + Ord,
    V: Clone + Hash + PartialEq + Eq,
{
    let mut igraph: HashMap<V, K> = HashMap::new();
    let mut keys: Vec<K> = graph.keys().cloned().collect();
    keys.sort();
    for key in keys.iter() {
        if let Some(value) = graph.get(key) {
            if !igraph.contains_key(value) {
                igraph.insert(value.clone(), key.clone());
            }
        }
    }
    return igraph;
}

/// computes sum of digits (base 10) in number
fn diagonalise<T>(x: T) -> T
where
    T: ToString + FromStr + Add<Output = T>,
    <T as FromStr>::Err: Debug,
{
    let zero: T = "0".parse::<T>().unwrap();
    return x
        .to_string()
        .chars()
        .map(|u| u.to_string())
        .map(|u| u.parse::<T>())
        .flatten()
        .fold(zero, |prev, d| prev + d);
}

/// ----------------------------------------------------------------
/// Helper structs/traits
/// ----------------------------------------------------------------

/// Helper structure for modulo computations
#[derive(Clone, Copy, PartialEq, Eq)]
struct SpecialNumber<T>
where
    T: Clone + Copy + PartialEq + Eq,
{
    value: T,
    modulus: Option<T>,
    max_value: Option<T>,
}

impl<T> SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq,
{
    fn new(value: T, modulus: Option<T>, max_value: Option<T>) -> Self {
        Self { value, modulus, max_value }
    }

    /// creates an object like the current one but with a different "value"
    fn mould(&self, value: T) -> Self {
        Self::new(value, self.modulus, self.max_value)
    }

    fn generator(modulus: Option<T>, max_value: Option<T>) -> Box<dyn Fn(T) -> Self>
    where
        T: 'static,
    {
        let fct = move |value| Self::new(value, modulus, max_value);
        Box::new(fct)
    }
}

impl SpecialNumber<i64> {
    #[allow(unused)]
    fn digits(&self) -> Vec<Self> {
        let fct = SpecialNumber::<i64>::generator(self.modulus, self.max_value);
        self.value
            .to_string()
            .chars()
            .map(|u| u.to_string())
            .map(|u| u.parse::<i64>())
            .flatten()
            .map(fct)
            .collect()
    }

    /// Computes the sum of the digits in the expansion
    #[allow(unused)]
    fn diagonalise(&self) -> SpecialNumber<i64> {
        let zero = self.mould(0);
        let sum = self.digits().iter().fold(zero, |s, &xx| s + xx);
        return sum;
    }
}

impl<T> Display for SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.modulus {
            Some(modulus) => write!(f, "{} (mod {})", self.value, modulus),
            None => write!(f, "{}", self.value),
        }
    }
}

impl<T> Debug for SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> Add for SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq + Ord + Add<Output = T> + Rem<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut value = self.value;
        match (self.max_value, self.modulus) {
            (Some(max_value), Some(modulus)) => {
                // nothing to do if already exceeds capacity
                if value >= max_value {
                    return self.clone();
                }
                value = value.max((value + other.value) % modulus);
            }
            (Some(max_value), None) => {
                // nothing to do if already exceeds capacity
                if value >= max_value {
                    return self.clone();
                }
                value = value.max(value + other.value);
            }
            (None, Some(modulus)) => {
                value = (value + other.value) % modulus;
            }
            _ => {
                value = value + other.value;
            }
        }
        return self.mould(value);
    }
}

impl<T> Mul for SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq + Ord + Mul<Output = T> + Rem<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut value = self.value;
        match (self.max_value, self.modulus) {
            (Some(max_value), Some(modulus)) => {
                // nothing to do if already exceeds capacity
                if value >= max_value {
                    return self.clone();
                }
                value = value.max((value * other.value) % modulus);
            }
            (Some(max_value), None) => {
                // nothing to do if already exceeds capacity
                if value >= max_value {
                    return self.clone();
                }
                value = value.max(value * other.value);
            }
            (None, Some(modulus)) => {
                value = (value * other.value) % modulus;
            }
            _ => {
                value = value * other.value;
            }
        }
        return self.mould(value);
    }
}

impl<T> AddAssign for SpecialNumber<T>
where
    T: Display + Clone + Copy + PartialEq + Eq + Ord + Add<Output = T> + Rem<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        let x = self.clone() + rhs;
        self.value = x.value;
        self.modulus = x.modulus;
    }
}

impl<T> MulAssign for SpecialNumber<T>
where
    T: Display
        + Clone
        + Copy
        + PartialEq
        + Eq
        + Ord
        + Mul<Output = T>
        + Add<Output = T>
        + Rem<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        let x = self.clone() * rhs;
        self.value = x.value;
        self.modulus = x.modulus;
    }
}

/// ----------------------------------------------------------------
/// AUXILIARY
/// ----------------------------------------------------------------

/// Obtains input lines from stdin
/// as a vector of strings.
#[allow(unused)]
fn read_input(stream: &Stdin) -> Vec<String> {
    stream.lock().lines().filter_map(Result::ok).collect()
}

#[allow(unused)]
fn parse<T>(text: &String) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    return text.parse::<T>().unwrap();
}

pub fn sort_hash<K, V>(map: HashMap<K, V>) -> Vec<(K, V)>
where
    K: Clone + Hash + Ord,
    V: Clone,
{
    let mut keys: Vec<K> = map.keys().cloned().collect();
    keys.sort();
    keys.iter()
        .map(|key| (key.clone(), map.get(key).unwrap().clone()))
        .collect()
}
