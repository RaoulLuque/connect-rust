mod bruteforce_helpers;

use std::ops::BitXor;

use crate::gamestate_helpers::{
    is_over, is_won, possible_next_gamestates, whos_turn_is_it_gamestate, PlayerColor,
};
use bruteforce_helpers::mirror_gamestate;
use connect_rust_graphs::graph::Graph;

pub struct Engine {
    color: PlayerColor,

    // Visits for tracking progress
    #[allow(dead_code)]
    visits: u128,
}

impl Engine {
    pub fn new(color: PlayerColor) -> Engine {
        Engine { color, visits: 0 }
    }

    pub fn make_move(&self, current_gamestate: u128) -> u128 {
        possible_next_gamestates(current_gamestate).last().unwrap()
    }
}
