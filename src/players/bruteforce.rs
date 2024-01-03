mod move_ordering;
mod negamax;
mod opening_gamestate_lookup_table;
mod transposition_table;

use crate::helpers::moves::{compute_winning_positions, get_one_of_the_bits, possible_moves};
use crate::helpers::turns::{number_of_turns_played, whos_turn_is_it_gamestate};
use negamax::{negamax_with_gamestate, HEIGHT, WIDTH};
use opening_gamestate_lookup_table::opening_moves;
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

        // Return winning move immediately if exists
        let instant_winning_moves =
            compute_winning_positions(current_gamestate, color) & possible_moves(current_gamestate);
        if (instant_winning_moves).count_ones() > 0 {
            return (
                get_one_of_the_bits(instant_winning_moves) | current_gamestate,
                (7 * 6 + 1 - number_of_turns_played) / 2,
                number_of_visited_nodes,
                time.elapsed().as_micros(),
            );
        }

        if current_gamestate.count_ones() <= 3 {
            return (
                opening_moves(current_gamestate),
                0,
                number_of_visited_nodes,
                time.elapsed().as_micros(),
            );
        }

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
        );
        assert_eq!(
            Engine::make_move(6522890914424695115743360).0,
            6522891490885447419166848
        )
    }
}
