use std::{
    ops::{AddAssign, BitOr},
    thread::current,
};

use crate::helpers::{
    encoding_gamestates::turn_column_to_encoded_gamestate, moves::is_winning_move,
    state_of_game::is_full, turns::number_of_turns_played, PlayerColor,
};

use super::transposition_table::{self, TranspositionTable};

pub const WIDTH: i8 = 7;
pub const HEIGHT: i8 = 6;
const MIN_SCORE: i8 = -(WIDTH * HEIGHT) / 2 + 3;
const ITERATE: [u8; 7] = [4, 3, 5, 2, 6, 1, 7];

/// returns rating of function
pub fn negamax(
    current_gamestate: u128,
    mut alpha: i8,
    mut beta: i8,
    color: PlayerColor,
    number_of_visits: &mut u32,
    ) -> i8 {
    number_of_visits.add_assign(1);

    if is_full(current_gamestate) {
        return 0;
    }

    for column in ITERATE.iter() {
        if is_winning_move(current_gamestate, *column) {
            number_of_visits.add_assign(1);
            return (WIDTH * HEIGHT + 1 - number_of_turns_played(current_gamestate) as i8)
                .div_euclid(2);
        }
    }

    let max: i8 = (WIDTH * HEIGHT - 1 - (number_of_turns_played(current_gamestate) as i8)) / 2;
    if beta > max {
        beta = max;
        if alpha >= beta {
            // Prune if exploration window [alpha : beta] is empty
            return beta;
        }
    }

    for column in ITERATE.iter() {
        if let Some((gamestate, _)) =
            turn_column_to_encoded_gamestate(current_gamestate, *column as u32, &color)
        {
            let score: i8 = -match color {
                PlayerColor::Blue => negamax(
                    gamestate.bitor(current_gamestate),
                    -beta,
                    -alpha,
                    PlayerColor::Red,
                    number_of_visits,
                                    ),
                PlayerColor::Red => negamax(
                    gamestate.bitor(current_gamestate),
                    -beta,
                    -alpha,
                    PlayerColor::Blue,
                    number_of_visits,
                                    ),
            };

            if score >= beta {
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }
    }

        alpha
}
