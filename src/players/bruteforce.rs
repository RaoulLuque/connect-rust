mod negamax;

use crate::helpers::turns::{number_of_turns_played, whos_turn_is_it_gamestate};
use negamax::{negamax, HEIGHT, WIDTH};

use std::time::Instant;

// Whether to solve weakly or strongly
const WEAK_SOLVE: bool = false;

pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        let time = Instant::now();
        let mut number_of_visited_nodes: u32 = 0;

        let (min, max) = match WEAK_SOLVE {
            false => (
                -(WIDTH * HEIGHT - (number_of_turns_played(current_gamestate) as i8)) / 2,
                (WIDTH * HEIGHT - (number_of_turns_played(current_gamestate) as i8)) / 2,
            ),
            true => (-1, 1),
        };

        let evaluation: i8 = negamax(
            current_gamestate,
            min,
            max,
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
