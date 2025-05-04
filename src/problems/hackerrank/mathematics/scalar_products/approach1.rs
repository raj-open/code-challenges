use core::iter::IntoIterator;
use core::iter::Iterator;
/// # First approach #
///
/// This approach is not mathematically optimised,
/// but rather designe do provid a more "rust native" approach.
///
/// In paricular we rely on constructing iterables.

/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------
// use core::convert::TryFrom;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::slice::Iter;
use std::str::FromStr;

/// ----------------------------------------------------------------
/// MAIN
/// ----------------------------------------------------------------

#[allow(unused)]
fn main() {
    let line = read_input();

    let args: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
    let mut args: Iter<'_, String> = args.iter();

    let c = parse::<i32>(args.next().unwrap());
    let m = parse::<i32>(args.next().unwrap());
    let n = parse::<usize>(args.next().unwrap());

    let num_unique = run(c, m, n);

    println!("{:?}", num_unique);
}

pub fn run(c: i32, m: i32, n: usize) -> usize {
    let vecs = SeqPair::new(m, 0, c)
        .into_iter()
        .map(|s| (s.current, s.next))
        .skip(2)
        .step_by(2)
        .take(n);

    let mut values: HashSet<i32> = HashSet::new();

    for (k, u) in vecs.enumerate() {
        let vecs2 = SeqPair::new(m, u.0, u.1)
            .into_iter()
            .map(|s| (s.current, s.next))
            .skip(2)
            .step_by(2)
            .take(n - (k + 1));
        for v in vecs2 {
            let ip = (u.0 * v.0 + u.1 * v.1).rem_euclid(m);
            values.insert(ip);
        }
    }
    let num_unique = values.len();

    return num_unique;
}

/// ----------------------------------------------------------------
/// SECONDARY
/// ----------------------------------------------------------------

#[derive(Clone, Debug)]
struct SeqPair {
    modulus: i32,
    current: i32,
    next: i32,
}

trait EntityIterable {
    fn next_entity(&mut self) -> &Self;
}

#[derive(Clone, Debug)]
struct EntityIterator<T>
where
    T: Clone + Debug + EntityIterable,
{
    index: usize,
    entity: T,
}

impl SeqPair {
    fn new(modulus: i32, x: i32, y: i32) -> Self {
        Self {
            modulus,
            current: x,
            next: y,
        }
    }
}

impl ToString for SeqPair {
    fn to_string(&self) -> String {
        format!("({}, {})", self.current, self.next)
    }
}

impl EntityIterable for SeqPair {
    fn next_entity(&mut self) -> &Self {
        let x = self.current;
        let y = self.next;
        self.current = y;
        self.next = (x + y).rem_euclid(self.modulus);
        return self;
    }
}

impl<T> Iterator for EntityIterator<T>
where
    T: Clone + Debug + EntityIterable,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index > 1 {
            self.entity.next_entity();
        }
        return Some(self.entity.clone());
    }
}

impl IntoIterator for SeqPair {
    type Item = SeqPair;
    type IntoIter = EntityIterator<SeqPair>;

    fn into_iter(self) -> EntityIterator<SeqPair> {
        EntityIterator {
            index: 0,
            entity: self.clone(),
        }
    }
}

/// ----------------------------------------------------------------
/// AUXILIARY
/// ----------------------------------------------------------------

#[allow(unused)]
fn read_input() -> String {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let line = input.next().unwrap().unwrap().trim().to_string();
    return line;
}

#[allow(unused)]
fn parse<T>(text: &String) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    return text.parse::<T>().unwrap();
}
