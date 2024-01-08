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
    pub fn make_move(current_gamestate: u128, using_lookup_table: bool) -> (u128, i8, u32, u128) {
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

        if current_gamestate.count_ones() <= 5 && using_lookup_table {
            return opening_moves(current_gamestate);
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

            let (evaluation, gamestate) = negamax_with_gamestate(
                current_gamestate,
                med,
                med + 1,
                color,
                &mut number_of_visited_nodes,
                &mut transposition_table,
            );

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
            Engine::make_move(9847905112306175136759808, false).0,
            9847905112306175673630720
        );
        assert_eq!(
            Engine::make_move(6522890914424695115743360, false).0,
            6522891490885447419166848
        )
    }

    #[test]
    fn make_move_starter_move_respond_correctly() {
        let (response, _, number_of_visited_nodes, time) =
            Engine::make_move(152296391130140096069632, true);

        println!("The time it took for 2nd turn to calculate: {}", time);
        println!(
            "The number of visited nodes for 2nd turn is: {}",
            number_of_visited_nodes
        );
        assert_eq!(response, 152305614502176950845440);

        let (response, _, number_of_visited_nodes, time) =
            Engine::make_move(1180591620717411303424, true);

        println!("The time it took for 1st turn to calculate: {}", time);
        println!(
            "The number of visited nodes for 1st turn is: {}",
            number_of_visited_nodes
        );
        assert_eq!(response, 152296319072546058141696);

        let (response, _, number_of_visited_nodes, time) =
            Engine::make_move(152305614783651927556096, true);

        println!("The time it took for 3rd turn to calculate: {}", time);
        println!(
            "The number of visited nodes for 3rd turn is: {}",
            number_of_visited_nodes
        );
        assert_eq!(response, 152305614783686287294464);
    }

    #[test]
    fn make_move_given_winning_move_for_enemy_place_there() {
        assert_eq!(
            Engine::make_move(152305614506574997356544, true).0,
            152305614506575534227456
        );
        assert_eq!(
            Engine::make_move(152305614506574997356544, false).0,
            152305614506575534227456
        );
    }

    #[test]
    fn make_move_given_possible_future_opening_for_enemy_close_it() {
        assert_eq!(Engine::make_move(1662374459625487653142528, false).0, 1662374461877287466827776);
        assert_eq!(Engine::make_move(94456553310939096547456, false).0, 0);
    }
}
