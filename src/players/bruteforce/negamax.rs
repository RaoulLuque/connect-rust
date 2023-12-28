use std::ops::AddAssign;

use crate::helpers::{
    encoding_gamestates::turn_column_to_encoded_gamestate,
    state_of_game::{is_over, is_won},
    turns::number_of_turns_played,
    PlayerColor,
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
        if let Some(gamestate) = turn_column_to_encoded_gamestate(current_gamestate, column, &color)
        {
            if let Some(_) = is_won(gamestate) {
                number_of_visits.add_assign(1);
                return WIDTH * HEIGHT + 1
                    - (number_of_turns_played(current_gamestate).div_euclid(2) as i8);
            }
        }
    }

    let mut best_score: i8 = -WIDTH * HEIGHT;

    for column in 1..8 {
        if let Some(gamestate) = turn_column_to_encoded_gamestate(current_gamestate, column, &color)
        {
            let score: i8 = match color {
                PlayerColor::Blue => negamax(gamestate, PlayerColor::Red, number_of_visits),
                PlayerColor::Red => negamax(gamestate, PlayerColor::Blue, number_of_visits),
            };

            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}
