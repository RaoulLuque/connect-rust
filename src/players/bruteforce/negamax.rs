use std::collections::HashMap;
use std::ops::AddAssign;

use crate::helpers::{
    moves::{calculate_non_losing_moves, possible_next_gamestates},
    turns::number_of_turns_played,
    PlayerColor,
};

use super::move_ordering::move_score;

pub const WIDTH: i8 = 7;
pub const HEIGHT: i8 = 6;
const MIN_SCORE: i8 = -(WIDTH * HEIGHT) / 2 + 3;

/// returns rating of function
pub fn negamax(
    current_gamestate: u128,
    mut alpha: i8,
    mut beta: i8,
    color: PlayerColor,
    number_of_visits: &mut u32,
    transposition_table: &mut HashMap<u128, i8>,
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
    if alpha < min {
        alpha = min;
        if alpha >= beta {
            return alpha;
        }
    }

    let mut max: i8 = (WIDTH * HEIGHT - 1 - (number_of_turns_played)) / 2;

    let mut was_in_transposition_table: bool = false;
    if let Some(value) = transposition_table.get(&current_gamestate) {
        max = *value + MIN_SCORE - 1;
        was_in_transposition_table = true;
    }

    if beta > max {
        beta = max;
        if alpha >= beta {
            // Prune if exploration window [alpha : beta] is empty
            return beta;
        }
    }

    let mut next_gamestates_sorted: Vec<u128> =
        possible_next_gamestates(current_gamestate).collect();
    next_gamestates_sorted.sort_by(|a, b| move_score(*b, color).cmp(&move_score(*a, color)));

    for next_gamestate in next_gamestates_sorted {
        if (next_gamestate & next_possible_moves).count_ones() > 0 {
            let score: i8 = -match color {
                PlayerColor::Blue => negamax(
                    next_gamestate,
                    -beta,
                    -alpha,
                    PlayerColor::Red,
                    number_of_visits,
                    transposition_table,
                ),
                PlayerColor::Red => negamax(
                    next_gamestate,
                    -beta,
                    -alpha,
                    PlayerColor::Blue,
                    number_of_visits,
                    transposition_table,
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
    if !was_in_transposition_table {
        transposition_table.insert(current_gamestate, alpha - MIN_SCORE + 1);
    }
    alpha
}

/// returns rating of function
pub fn negamax_with_gamestate(
    current_gamestate: u128,
    mut alpha: i8,
    mut beta: i8,
    color: PlayerColor,
    number_of_visits: &mut u32,
    transposition_table: &mut HashMap<u128, i8>,
) -> (i8, u128) {
    number_of_visits.add_assign(1);

    let number_of_turns_played: i8 = number_of_turns_played(current_gamestate) as i8;
    let random_move: u128 = possible_next_gamestates(current_gamestate).next().unwrap();

    let next_possible_moves = calculate_non_losing_moves(current_gamestate, color);
    if next_possible_moves == 0 {
        // every move will loose the game
        return (-(WIDTH * HEIGHT - number_of_turns_played) / 2, random_move);
    }

    if current_gamestate.count_ones() >= ((WIDTH * HEIGHT) - 2) as u32 {
        return (0, random_move);
    }

    let min: i8 = -(WIDTH * HEIGHT - 2 - number_of_turns_played) / 2;
    if alpha < min {
        alpha = min;
        if alpha >= beta {
            return (alpha, random_move);
        }
    }

    let mut max: i8 = (WIDTH * HEIGHT - 1 - (number_of_turns_played)) / 2;

    let mut was_in_transposition_table: bool = false;
    if let Some(value) = transposition_table.get(&current_gamestate) {
        max = *value + MIN_SCORE - 1;
        was_in_transposition_table = true;
    }

    if beta > max {
        beta = max;
        if alpha >= beta {
            // Prune if exploration window [alpha : beta] is empty
            return (beta, random_move);
        }
    }

    let mut next_gamestates_sorted: Vec<u128> =
        possible_next_gamestates(current_gamestate).collect();
    next_gamestates_sorted.sort_by(|a, b| move_score(*b, color).cmp(&move_score(*a, color)));

    let mut best_next_gamestate: u128 = 0;

    for next_gamestate in next_gamestates_sorted {
        if (next_gamestate & next_possible_moves).count_ones() > 0 {
            let score: i8 = -match color {
                PlayerColor::Blue => negamax(
                    next_gamestate,
                    -beta,
                    -alpha,
                    PlayerColor::Red,
                    number_of_visits,
                    transposition_table,
                ),
                PlayerColor::Red => negamax(
                    next_gamestate,
                    -beta,
                    -alpha,
                    PlayerColor::Blue,
                    number_of_visits,
                    transposition_table,
                ),
            };

            if best_next_gamestate == 0 {
                best_next_gamestate = next_gamestate;
            }

            if score >= beta {
                return (score, best_next_gamestate);
            }
            if score > alpha {
                alpha = score;
                best_next_gamestate = next_gamestate;
            }
        }
    }

    if best_next_gamestate == 0 {
        best_next_gamestate = random_move;
    }

    if !was_in_transposition_table {
        transposition_table.insert(current_gamestate, alpha - MIN_SCORE + 1);
    }

    (alpha, best_next_gamestate)
}
