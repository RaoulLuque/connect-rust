mod move_ordering;
mod negamax;
mod transposition_table;

use crate::helpers::turns::{number_of_turns_played, whos_turn_is_it_gamestate};
use negamax::{negamax, negamax_with_gamestate, HEIGHT, WIDTH};
use std::collections::HashMap;

use std::time::Instant;

// Whether to solve weakly or strongly
const WEAK_SOLVE: bool = false;

pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        // println!("Using bruteforce");

        let time = Instant::now();
        let mut number_of_visited_nodes: u32 = 0;
        let color = whos_turn_is_it_gamestate(current_gamestate);
        let number_of_turns_played: i8 = number_of_turns_played(current_gamestate) as i8;
        let mut best_next_gamestate: u128 = 0;
        let mut transposition_table: HashMap<u128, i8> = HashMap::new();

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

            let (evaluation, gamestate) = (negamax_with_gamestate(
                current_gamestate,
                med,
                med + 1,
                color,
                &mut number_of_visited_nodes,
                &mut transposition_table,
            ));

            if evaluation <= med {
                max = evaluation;
            } else {
                min = evaluation;
                best_next_gamestate = gamestate;
            }
        }

        (
            best_next_gamestate,
            min,
            number_of_visited_nodes,
            time.elapsed().as_micros(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;

    #[test]
    fn make_move_given_obvious_necessary_move_return_the_right_move() {
        assert_eq!(
            Engine::make_move(9847905112306175136759808).0,
            9847905112306175673630720
        )
    }
}
