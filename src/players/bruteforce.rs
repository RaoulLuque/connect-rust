mod negamax;
mod transposition_table;

use crate::helpers::turns::{number_of_turns_played, whos_turn_is_it_gamestate};
use negamax::{negamax, HEIGHT, WIDTH};
use transposition_table::TranspositionTable;

use std::time::Instant;

// Whether to solve weakly or strongly
const WEAK_SOLVE: bool = false;

pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        let time = Instant::now();
        let mut number_of_visited_nodes: u32 = 0;
        let color = whos_turn_is_it_gamestate(current_gamestate);
        let number_of_turns_played: i8 = number_of_turns_played(current_gamestate) as i8;

        let (mut min, mut max) = match WEAK_SOLVE {
            false => (
                -(WIDTH * HEIGHT - (number_of_turns_played)) / 2,
                (WIDTH * HEIGHT - (number_of_turns_played)) / 2,
            ),
            true => (-1, 1),
        };

        while min < max {
            let mut med = min + (max - min) / 2;

            if med <= 0 && min / 2 < med {
                med = min / 2
            } else if med >= 0 && max / 2 > med {
                med = max / 2
            }

            let evaluation = negamax(
                current_gamestate,
                med,
                med + 1,
                color,
                &mut number_of_visited_nodes,
            );

            if evaluation <= med {
                max = evaluation;
            } else {
                min = evaluation;
            }
        }
        (0, min, number_of_visited_nodes, time.elapsed().as_micros())
    }
}
