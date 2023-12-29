use std::{
    ops::{AddAssign, BitOr},
    thread::current,
};

use crate::helpers::{
    encoding_gamestates::turn_column_to_encoded_gamestate, moves::is_winning_move,
    state_of_game::is_over, turns::number_of_turns_played, PlayerColor,
};

const WIDTH: i8 = 7;
const HEIGHT: i8 = 6;

/// returns rating of function
pub fn negamax(current_gamestate: u128, color: PlayerColor, number_of_visits: &mut u32) -> i8 {
    number_of_visits.add_assign(1);

    if is_over(current_gamestate) {
        return 0;
    }

    for column in 1..8 {
        if is_winning_move(current_gamestate, column) {
            number_of_visits.add_assign(1);
            return WIDTH * HEIGHT + 1
                - (number_of_turns_played(current_gamestate).div_euclid(2) as i8);
        }
    }

    let mut best_score: i8 = -WIDTH * HEIGHT;

    for column in 1..8 {
        if let Some((gamestate, _)) =
            turn_column_to_encoded_gamestate(current_gamestate, column, &color)
        {
            let score: i8 = match color {
                PlayerColor::Blue => negamax(
                    gamestate.bitor(current_gamestate),
                    PlayerColor::Red,
                    number_of_visits,
                ),
                PlayerColor::Red => negamax(
                    gamestate.bitor(current_gamestate),
                    PlayerColor::Blue,
                    number_of_visits,
                ),
            };

            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}
