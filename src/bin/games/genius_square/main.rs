/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use std::env;

use general::app::messages::welcome_screen;
use general::_core;

mod algorithms;
mod models;
mod features;

use models::constants::dice::NUM_DICE;
use features::setup_game::feature_setup_game;

/// ----------------------------------------------------------------
/// MAIN
/// ----------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let option_roll = if args.len() >= NUM_DICE { Some(args[0..NUM_DICE].to_vec()) } else { None };
    let option_seed = if args.len() >= 1 { Some(args[args.len() - 1].clone()) } else { None };
    let mut rng = _core::rand::seed_rng(option_seed);
    welcome_screen();
    feature_setup_game(&mut rng, option_roll);
}
