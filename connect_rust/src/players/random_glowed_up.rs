use std::time::Duration;

use super::super::helpers::moves::{calculate_non_losing_moves, get_one_of_the_bits};
use super::random::Engine as Engine_random;
use crate::helpers::PlayerColor::Red;

/// The random_glowed_up/random* engine plays randomly except when there are three in a row for the human.
/// In which case the fourth token is placed to avoid loosing.
pub struct Engine;

impl Engine {
    /// A random move is returned if not loosing given an encoded gamestate.
    /// If the random move is not non loosing returns one of the non loosing moves.
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, Duration) {
        let move_from_random = Engine_random::make_move(current_gamestate);

        let non_loosing_moves = calculate_non_losing_moves(current_gamestate, Red);

        if (move_from_random.0 & non_loosing_moves) > 0 {
            // Case where random move is not instant loosing
            return move_from_random;
        } else {
            if non_loosing_moves > 0 {
                // Case where random move is instant loosing and there is a not instant loosing move
                let (_, evaluation, number_of_visited_nodes, time) = move_from_random;
                return (
                    current_gamestate | get_one_of_the_bits(non_loosing_moves),
                    evaluation,
                    number_of_visited_nodes,
                    time,
                );
            } else {
                // Case where random move is instant loosing and there are only instant loosing moves
                return move_from_random;
            }
        }
    }
}
