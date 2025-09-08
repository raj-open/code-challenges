// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use std::env;

use general::app::messages::welcome_screen;
use general::_core;

mod algorithms;
mod features;
mod models;

use models::constants::NUM_DICE;
use features::feature_roll_dice;
use features::feature_initialise_game;
use features::feature_solve_game;

// ----------------------------------------------------------------
// MAIN
// ----------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let option_roll = if args.len() >= NUM_DICE {
        Some(args[0..NUM_DICE].to_vec())
    } else {
        None
    };
    let option_seed = if !args.is_empty() {
        Some(args[args.len() - 1].clone())
    } else {
        None
    };
    let mut rng = _core::rand::seed_rng(option_seed);

    welcome_screen();
    let dice = feature_roll_dice(&mut rng, option_roll);
    let board = feature_initialise_game(&dice);
    feature_solve_game(&board);
}
