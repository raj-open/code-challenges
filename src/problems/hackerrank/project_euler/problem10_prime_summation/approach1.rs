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
use std::cmp::Ord;
use std::collections::HashMap;
use std::io::Stdin;
use std::str::FromStr;

/// ----------------------------------------------------------------
/// MAIN
/// ----------------------------------------------------------------

/// entry point when used as a script
#[allow(unused)]
fn main() {
    let lines = read_input(&io::stdin());
    let t = lines.iter().nth(0).unwrap().parse::<usize>();
    let numbers: Vec<i64> = lines.iter().skip(1).map(|x| x.parse().unwrap()).collect();

    let sums = run(&numbers);

    for n in numbers.iter() {
        if let Some(s) = sums.get(n) {
            println!("{:?}", s);
        }
    }
}

pub fn run(numbers: &Vec<i64>) -> HashMap<i64, i64> {
    let n_max: i64 = numbers.iter().fold(0, |prev, &n| prev.max(n));
    let primes = get_primes(n_max as i64);
    let sums = compute_aggregates(&numbers, &primes);
    return sums;
}

/// ----------------------------------------------------------------
/// HELPER METHODS
/// ----------------------------------------------------------------

/// computes the list of primes up to a value
fn get_primes(n_max: i64) -> Vec<i64> {
    let mut map = HashMap::<i64, bool>::new();
    let mut result = Vec::<i64>::new();
    for p in 2..(n_max + 1) {
        if map.get(&p) != Some(&false) {
            result.push(p);
            ((2 * p)..(n_max + 1)).step_by(p as usize).for_each(|k| {
                map.entry(k).or_insert(false);
            });
        }
    }
    return result;
}

fn compute_aggregates(numbers: &Vec<i64>, primes: &Vec<i64>) -> HashMap<i64, i64> {
    let mut numbers_sorted = numbers.clone();
    numbers_sorted.sort();
    numbers_sorted.dedup();
    let mut values = primes.clone();
    let mut sums = HashMap::<i64, i64>::new();
    let mut sum: i64 = 0;
    for n in numbers_sorted {
        sum += values.iter().filter(|&&p| (p <= n)).sum::<i64>();
        values = values.iter().filter(|&&p| (p > n)).cloned().collect();
        sums.insert(n, sum);
    }
    return sums;
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
