mod bruteforce_helpers;
mod negamax;

use crate::helpers::{moves::possible_next_gamestates, turns::whos_turn_is_it_gamestate};
use negamax::negamax;

use std::{thread::current, time::Instant};

pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        let time = Instant::now();
        let mut number_of_visited_nodes: u32 = 0;

        let evaluation = negamax(
            current_gamestate,
            -100,
            100,
            whos_turn_is_it_gamestate(current_gamestate),
            &mut number_of_visited_nodes,
        );

        (
            0,
            evaluation,
            number_of_visited_nodes,
            time.elapsed().as_micros(),
        )
    }
}
