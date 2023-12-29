mod bruteforce_helpers;
mod negamax;

use crate::helpers::{moves::possible_next_gamestates, turns::whos_turn_is_it_gamestate};
use negamax::negamax;

use std::time::Instant;

pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        let time = Instant::now();

        let mut current_best_evaluation: i8 = -56;
        let mut current_best_next_gamestate: u128 = 0;
        let mut number_of_visited_nodes: u32 = 0;

        for next_gamestate in possible_next_gamestates(current_gamestate) {
            if current_best_next_gamestate == 0 {
                current_best_next_gamestate = next_gamestate;
            }
            let evaluation = -negamax(
                next_gamestate,
                whos_turn_is_it_gamestate(next_gamestate),
                &mut number_of_visited_nodes,
            );

            if evaluation > current_best_evaluation {
                current_best_evaluation = evaluation;
                current_best_next_gamestate = current_gamestate;
            }
        }

        (
            current_best_next_gamestate,
            current_best_evaluation,
            number_of_visited_nodes,
            time.elapsed().as_micros(),
        )
    }
}
