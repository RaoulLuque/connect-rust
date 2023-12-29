use std::ops::BitXor;

use crate::response_handling::incoming::GameMoveInput;

use super::{
    encoding_gamestates::turn_column_to_encoded_gamestate, turns::whos_turn_is_it_gamestate, *,
};

// Constants for determining winning moves
const FULL_RED_ENCODED_BOARD: u128 = 12895208742556044530199210;
const FULL_BLUE_ENCODED_BOARD: u128 = 6447604371278022265099605;
const FULL_BOTH_COLOR_BOTTOM_ROW: u128 = 19341632522213349383995392;
const FULL_BOTH_COLOR_LEFT_SIDE: u128 = 301069239090989869547775;
const FULL_BOTH_COLOR_RIGHT_SIDE: u128 = 19268431301823351651057600;
const FULL_BOTH_COLOR_UP_RIGHT_BLOCK: u128 = 4381134045120;
const FULL_BOTH_COLOR_UP_LEFT_BLOCK: u128 = 68455219455;
const FULL_BOTH_COLOR_LOW_LEFT_BLOCK: u128 = 301069239090921414328320;
const FULL_BOTH_COLOR_LOW_RIGHT_BLOCK: u128 = 19268431301818970517012480;

/// Returns the possible next gamestates from a given gamestate as an iterator
pub fn possible_next_gamestates(
    current_gamestate: u128,
) -> std::collections::vec_deque::IntoIter<u128> {
    let mut res_queue: VecDeque<u128> = VecDeque::new();
    let player_whos_turn_it_is = whos_turn_is_it_gamestate(current_gamestate);

    // Add possible moves by checking all columns
    for column in 1..8 {
        let next_move =
            turn_column_to_encoded_gamestate(current_gamestate, column, &player_whos_turn_it_is);
        match next_move {
            Some((i, _)) => res_queue.push_back(i | current_gamestate),
            None => (),
        };
    }

    // Return iterator over possible moves
    res_queue.into_iter()
}

/// move_to_make is column that is supposed to be played
pub fn is_winning_move(gamestate: u128, move_to_make: u8) -> bool {
    if let Some((move_encoded, row_that_was_placed)) = turn_column_to_encoded_gamestate(
        gamestate,
        move_to_make as u32,
        &whos_turn_is_it_gamestate(gamestate),
    ) {
        if check_horizontal_row(gamestate, move_encoded, move_to_make)
            || check_vertical_row(gamestate, move_encoded, row_that_was_placed)
        {
            return true;
        } else if check_lowerleft_upperright_diagonal(
            gamestate,
            move_encoded,
            row_that_was_placed,
            move_to_make,
        ) || check_upperleft_lowerright_diagonal(
            gamestate,
            move_encoded,
            row_that_was_placed,
            move_to_make,
        ) {
            return true;
        } else {
            return false;
        }
    }

    false
}

pub fn check_lowerleft_upperright_diagonal(
    gamestate: u128,
    move_encoded: u128,
    row_of_move: u8,
    column_of_move: u8,
) -> bool {
    let mut move_encoded_copy = move_encoded;
    // Counter for how many matching tokens are found left and right of the new token (move)
    let mut in_a_row: u8 = 0;

    let left_bound: u8 = (column_of_move - 1).min(row_of_move - 1);
    let right_bound: u8 = (7 - column_of_move).min(6 - row_of_move);

    // Look left
    for _ in 0..left_bound {
        move_encoded_copy *= BASE.pow(12);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    let mut move_encoded_copy = move_encoded;
    // Look right
    for _ in 0..right_bound {
        move_encoded_copy /= BASE.pow(12);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    if in_a_row >= 3 {
        true
    } else {
        false
    }
}

pub fn check_upperleft_lowerright_diagonal(
    gamestate: u128,
    move_encoded: u128,
    row_of_move: u8,
    column_of_move: u8,
) -> bool {
    let mut move_encoded_copy = move_encoded;
    // Counter for how many matching tokens are found left and right of the new token (move)
    let mut in_a_row: u8 = 0;

    let left_bound: u8 = (column_of_move - 1).min(6 - row_of_move);
    let right_bound: u8 = (7 - column_of_move).min(row_of_move - 1);

    // Look left
    for _ in 0..left_bound {
        move_encoded_copy /= BASE.pow(16);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    let mut move_encoded_copy = move_encoded;
    // Look right
    for _ in 0..right_bound {
        move_encoded_copy *= BASE.pow(16);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    if in_a_row >= 3 {
        true
    } else {
        false
    }
}

pub fn check_horizontal_row(gamestate: u128, move_encoded: u128, column_of_move: u8) -> bool {
    let mut move_encoded_copy = move_encoded;
    // Counter for how many matching tokens are found left and right of the new token (move)
    let mut in_a_row: u8 = 0;

    let left_bound: u8 = column_of_move - 1;
    let right_bound: u8 = 7 - column_of_move;

    // Look left
    for _ in 0..left_bound {
        move_encoded_copy /= BASE.pow(2);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    let mut move_encoded_copy = move_encoded;
    // Look right
    for _ in 0..right_bound {
        move_encoded_copy *= BASE.pow(2);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    if in_a_row >= 3 {
        true
    } else {
        false
    }
}

pub fn check_vertical_row(gamestate: u128, move_encoded: u128, row_of_move: u8) -> bool {
    let mut move_encoded_copy = move_encoded;
    // Counter for how many matching tokens are found left and right of the new token (move)
    let mut in_a_row: u8 = 0;

    let down_bound: u8 = row_of_move - 1;

    // Look down
    for _ in 0..down_bound {
        move_encoded_copy *= BASE.pow(14);
        if move_encoded_copy & gamestate != move_encoded_copy {
            break;
        } else {
            in_a_row += 1;
        }
    }

    if in_a_row >= 3 {
        true
    } else {
        false
    }
}

/// Returns the encoding of the gamestate where there are 1's for both color where there is a token
/// for one of the colors
pub fn gamestate_full(gamestate: u128) -> u128 {
    gamestate
        | ((gamestate & FULL_BLUE_ENCODED_BOARD) << 1)
        | ((gamestate & FULL_RED_ENCODED_BOARD) >> 1)
}

/// Returns the encoding of the possible moves (moves without rest of board - for both colors at once)
pub fn possible_moves(gamestate: u128) -> u128 {
    // Everywhere where a token of one color is, both color bits are 1
    let gamestate_full = gamestate_full(gamestate);

    (gamestate_full >> 14 & !(gamestate_full))
        | (gamestate_full.bitxor(FULL_BOTH_COLOR_BOTTOM_ROW) & FULL_BOTH_COLOR_BOTTOM_ROW)
}

/// Returns the encoding of the possible tokens that would make the opponent win immediately,
/// no matter if they are reachable
pub fn opponent_winning_positions(gamestate: u128, color: PlayerColor) -> u128 {
    match color {
        PlayerColor::Blue => compute_winning_positions(gamestate, PlayerColor::Red),
        PlayerColor::Red => compute_winning_positions(gamestate, PlayerColor::Blue),
    }
}

/// Returns the encoding of the possible tokens that would make the given color win immediately,
/// no matter if they are reachable
pub fn compute_winning_positions(gamestate: u128, color: PlayerColor) -> u128 {
    let full_board: u128 = match color {
        PlayerColor::Blue => FULL_BLUE_ENCODED_BOARD,
        PlayerColor::Red => FULL_RED_ENCODED_BOARD,
    };

    let mut winning_positions: u128 = 0;

    let gamestate = full_board & gamestate;

    // vertical
    winning_positions |= (gamestate >> 14) & (gamestate >> 28) & (gamestate >> 42);

    // horizontal
    winning_positions |=
        ((gamestate >> 2) & (gamestate >> 4) & (gamestate >> 6)) & FULL_BOTH_COLOR_LEFT_SIDE;
    winning_positions +=
        ((gamestate << 2) & (gamestate << 4) & (gamestate << 6)) & FULL_BOTH_COLOR_RIGHT_SIDE;

    // diagonal lowleft to upright
    winning_positions |= ((gamestate >> 12) & (gamestate >> 24) & (gamestate >> 36))
        & FULL_BOTH_COLOR_UP_RIGHT_BLOCK;

    // diagonal lowright to upleft
    winning_positions |=
        ((gamestate >> 16) & (gamestate >> 32) & (gamestate >> 48)) & FULL_BOTH_COLOR_UP_LEFT_BLOCK;

    // diagonal upright to lowleft
    winning_positions |= ((gamestate << 12) & (gamestate << 24) & (gamestate << 36))
        & FULL_BOTH_COLOR_LOW_LEFT_BLOCK;

    // diagonal upleft to lowright
    winning_positions |= ((gamestate << 16) & (gamestate << 32) & (gamestate << 48))
        & FULL_BOTH_COLOR_LOW_RIGHT_BLOCK;

    winning_positions
}

/// Returns the encoding of the possible moves/tokens that would make the given color not loose
/// immediately for the current gamestate (all of such moves without the rest of the gamestate).
///
/// Returns 0 if there are none.
pub fn calculate_non_losing_moves(gamestate: u128, color: PlayerColor) -> u128 {
    let possible_moves = possible_moves(gamestate);
    let opponent_winning_moves = opponent_winning_positions(gamestate, color);
    let forced_moves = possible_moves & opponent_winning_moves;
    if forced_moves.count_ones() > 1 {
        0
    } else if forced_moves.count_ones() == 1 {
        forced_moves & !(opponent_winning_moves << 14)
    } else {
        possible_moves & !(opponent_winning_moves << 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_next_gamestates_given_gamestate_return_next_gamestate_is_in_iterator() {
        let vec: Vec<u128> = possible_next_gamestates(BASE.pow(82)).collect();
        assert!(vec.contains(&(BASE.pow(81) + BASE.pow(82))));
    }

    #[test]
    fn check_lower_left_diagonal_given_winning_lower_left_diagonal_return_true() {
        assert_eq!(
            check_lowerleft_upperright_diagonal(18894078743396915085312, 274877906944, 4, 6),
            true
        );
    }

    #[test]
    fn check_lower_left_diagonal_given_not_winning_lower_left_diagonal_return_false() {
        assert_eq!(
            check_lowerleft_upperright_diagonal(302236066660044465242112, 1073741824, 4, 2),
            false
        );
    }

    #[test]
    fn check_lower_right_diagonal_given_winning_lower_right_diagonal_return_true() {
        assert_eq!(
            check_upperleft_lowerright_diagonal(302236066660044465242112, 1073741824, 4, 2),
            true
        );
    }

    #[test]
    fn check_lower_right_diagonal_given_not_winning_lower_right_diagonal_return_false() {
        assert_eq!(
            check_upperleft_lowerright_diagonal(18894078743396915085312, 274877906944, 4, 6),
            false
        );
    }

    #[test]
    fn is_winning_move_given_winnning_move_return_true() {
        assert_eq!(is_winning_move(6825767598171467010101410, 2), true)
    }

    #[test]
    fn gamestate_full_given_bottom_row_blue_return_bottom_blue_filled() {
        assert_eq!(
            gamestate_full(6447210840737783127998464),
            FULL_BOTH_COLOR_BOTTOM_ROW
        )
    }

    #[test]
    fn possible_moves_given_bottom_row_blue_return_next_row_both_colors_filled() {
        assert_eq!(
            possible_moves(6447210840737783127998464),
            1180519563123373375488
        )
    }

    #[test]
    fn compute_winning_positions_given_cross_return_outer_cross() {
        assert_eq!(
            compute_winning_positions(4613163779234988032, PlayerColor::Blue),
            4521191814463488
        )
    }

    #[test]
    fn compute_winning_positions_given_opportunities_return_winning_moves() {
        assert_eq!(
            compute_winning_positions(75229342058982408192, PlayerColor::Blue),
            4836883870079303102562304
        )
    }

    #[test]
    fn opponent_winning_positions_given_opportunities_return_winning_moves() {
        assert_eq!(
            opponent_winning_positions(75229342058982408192, PlayerColor::Red),
            4836883870079303102562304
        );
        assert_eq!(
            opponent_winning_positions(4613163779234988032, PlayerColor::Red),
            4521191814463488
        )
    }
}
