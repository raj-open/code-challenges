/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::env;

use general::_core;

mod algorithms;
mod models;
mod features;

use features::setup_game::feature_setup_game;

/// ----------------------------------------------------------------
/// MAIN
/// ----------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let option_roll = if args.len() >= 7 { Some(args[0..7].to_vec()) } else { None };
    let option_seed = if args.len() >= 1 { Some(args[args.len() - 1].clone()) } else { None };
    let mut rng = _core::rand::seed_rng(option_seed);
    feature_setup_game(&mut rng, option_roll);
}
