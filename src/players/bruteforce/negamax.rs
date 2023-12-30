use std::{
    ops::{AddAssign, BitOr},
    thread::current,
};

use crate::helpers::{
    encoding_gamestates::turn_column_to_encoded_gamestate,
    moves::{calculate_non_losing_moves, is_winning_move},
    state_of_game::is_full,
    turns::number_of_turns_played,
    PlayerColor,
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

    let number_of_turns_played: i8 = number_of_turns_played(current_gamestate) as i8;

    let next_possible_moves = calculate_non_losing_moves(current_gamestate, color);
    if next_possible_moves == 0 {
        // every move will loose the game
        return -(WIDTH * HEIGHT - number_of_turns_played) / 2;
    }

    if current_gamestate.count_ones() >= ((WIDTH * HEIGHT) - 2) as u32 {
        return 0;
    }

    let min: i8 = -(WIDTH * HEIGHT - 2 - number_of_turns_played) / 2;
    if (alpha < min) {
        alpha = min;
        if alpha >= beta {
            return alpha;
        }
    }

    let max: i8 = (WIDTH * HEIGHT - 1 - (number_of_turns_played)) / 2;
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
            if (gamestate & next_possible_moves).count_ones() > 0 {
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
    }

    alpha
}
