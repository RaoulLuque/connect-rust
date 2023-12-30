use std::ops::BitXor;

use super::{
    encoding_gamestates::turn_column_to_encoded_gamestate, turns::whos_turn_is_it_gamestate, *,
};

// Constants for determining winning moves
const FULL_RED_ENCODED_BOARD: u128 = 12895208742556044530199210;
const FULL_BLUE_ENCODED_BOARD: u128 = 6447604371278022265099605;
const FULL_BOTH_COLOR_BOTTOM_ROW: u128 = 19341632522213349383995392;
const FULL_BOTH_COLOR_LEFT_SIDE: u128 = 301069239090989869547775;
const FULL_BOTH_COLOR_LEFT_SIDE_ONE_COLUMN_FREE: u128 = 1204276956363959478191100;
const FULL_BOTH_COLOR_RIGHT_SIDE: u128 = 19268431301823351651057600;
const FULL_BOTH_COLOR_RIGHT_SIDE_ONE_COLUMN_FREE: u128 = 4817107825455837912764400;
const FULL_BOTH_COLOR_UP_RIGHT_BLOCK: u128 = 4381134045120;
const FULL_BOTH_COLOR_UP_RIGHT_BLOCK_ONE_TOWARDS_MIDDLE: u128 = 17945125048811520;
const FULL_BOTH_COLOR_UP_LEFT_BLOCK: u128 = 68455219455;
const FULL_BOTH_COLOR_UP_LEFT_BLOCK_ONE_TOWARDS_MIDDLE: u128 = 4486281262202880;
const FULL_BOTH_COLOR_LOW_LEFT_BLOCK: u128 = 301069239090921414328320;
const FULL_BOTH_COLOR_LOW_LEFT_BLOCK_ONE_TOWARDS_MIDDLE: u128 = 73503232199931985920;
const FULL_BOTH_COLOR_LOW_RIGHT_BLOCK: u128 = 19268431301818970517012480;
const FULL_BOTH_COLOR_LOW_RIGHT_BLOCK_ONE_TOWARDS_MIDDLE: u128 = 294012928799727943680;

// Constant for finding a bit
const FULL_BOTH_COLOR_LEFT_COLUMN: u128 = 3541991048129292582915;

// Basic move ordering
const ITERATE: [u8; 7] = [4, 3, 5, 2, 6, 1, 7];

/// Returns the possible next gamestates from a given gamestate as an iterator
pub fn possible_next_gamestates(
    current_gamestate: u128,
) -> std::collections::vec_deque::IntoIter<u128> {
    let mut res_queue: VecDeque<u128> = VecDeque::new();
    let player_whos_turn_it_is = whos_turn_is_it_gamestate(current_gamestate);

    // Add possible moves by checking all columns
    for column in ITERATE.iter() {
        let next_move = turn_column_to_encoded_gamestate(
            current_gamestate,
            *column as u32,
            &player_whos_turn_it_is,
        );
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
    let gamestate_full = gamestate_full(gamestate);
    let gamestate = full_board & gamestate;

    // For drawings m is the missing token and x are the ones there already
    // vertical
    // (m x x x)^T
    winning_positions |= (gamestate >> 14) & (gamestate >> 28) & (gamestate >> 42);

    // horizontal
    // left
    let left = (gamestate >> 2) & (gamestate >> 4);
    // m x x x
    winning_positions |= (left & (gamestate >> 6)) & FULL_BOTH_COLOR_LEFT_SIDE;
    // x m x x
    winning_positions |=
        (left & (gamestate * BASE.pow(2))) & FULL_BOTH_COLOR_LEFT_SIDE_ONE_COLUMN_FREE;

    // right
    let right = (gamestate * BASE.pow(2)) & (gamestate * BASE.pow(4));
    // x x x m
    winning_positions |= (right & (gamestate * BASE.pow(6))) & FULL_BOTH_COLOR_RIGHT_SIDE;
    // x x m x
    winning_positions |= (right & (gamestate >> 2)) & FULL_BOTH_COLOR_RIGHT_SIDE_ONE_COLUMN_FREE;

    // diagonal lowleft to upright
    let left = (gamestate >> 12) & (gamestate >> 24);
    // o o o m
    // o o x o
    // o x o o
    // x o o o
    winning_positions |= (left & (gamestate >> 36)) & FULL_BOTH_COLOR_UP_RIGHT_BLOCK;
    // o o o x
    // o o m o
    // o x o o
    // x o o o
    winning_positions |=
        (left & (gamestate << 12)) & FULL_BOTH_COLOR_UP_RIGHT_BLOCK_ONE_TOWARDS_MIDDLE;

    // diagonal lowright to upleft
    let right = (gamestate >> 16) & (gamestate >> 32);
    // m o o o
    // o x o o
    // o o x o
    // o o o x
    winning_positions |= (right & (gamestate >> 48)) & FULL_BOTH_COLOR_UP_LEFT_BLOCK;
    // x o o o
    // o m o o
    // o o x o
    // o o o x
    winning_positions |=
        (right & (gamestate << 16)) & FULL_BOTH_COLOR_UP_LEFT_BLOCK_ONE_TOWARDS_MIDDLE;

    // diagonal upright to lowleft
    let right = (gamestate << 12) & (gamestate << 24);
    // o o o x
    // o o x o
    // o x o o
    // m o o o
    winning_positions |= (right & (gamestate << 36)) & FULL_BOTH_COLOR_LOW_LEFT_BLOCK;
    // o o o x
    // o o x o
    // o m o o
    // x o o o
    winning_positions |=
        (right & (gamestate >> 12)) & FULL_BOTH_COLOR_LOW_LEFT_BLOCK_ONE_TOWARDS_MIDDLE;

    // diagonal upleft to lowright
    let left = (gamestate << 16) & (gamestate << 32);
    // x o o o
    // o x o o
    // o o x o
    // o o o m
    winning_positions |= (left & (gamestate << 48)) & FULL_BOTH_COLOR_LOW_RIGHT_BLOCK;
    // x o o o
    // o x o o
    // o o m o
    // o o o x
    winning_positions |=
        (left & (gamestate >> 16)) & FULL_BOTH_COLOR_LOW_RIGHT_BLOCK_ONE_TOWARDS_MIDDLE;

    winning_positions & !gamestate_full
}

/// Returns the encoding of the possible moves/tokens that would make the given color not loose
/// immediately for the current gamestate (all of such moves without the rest of the gamestate).
///
/// Returns 0 if there are none.
pub fn calculate_non_losing_moves(gamestate: u128, color: PlayerColor) -> u128 {
    let possible_moves = possible_moves(gamestate);
    let opponent_winning_moves = opponent_winning_positions(gamestate, color);
    // Necessary in order to avoid enemy being able to win immediately

    let (forced_moves, opponent_winning_moves) = match color {
        PlayerColor::Red => (
            possible_moves & (opponent_winning_moves * 2),
            opponent_winning_moves * 2,
        ),
        PlayerColor::Blue => (
            possible_moves & (opponent_winning_moves >> 1),
            opponent_winning_moves >> 1,
        ),
    };

    let moves = if forced_moves.count_ones() > 1 {
        0
    } else if forced_moves.count_ones() == 1 {
        forced_moves & !(opponent_winning_moves * BASE.pow(14))
    } else {
        possible_moves & !(opponent_winning_moves * BASE.pow(14))
    };
    match color {
        PlayerColor::Blue => moves & FULL_BLUE_ENCODED_BOARD,
        PlayerColor::Red => moves & FULL_RED_ENCODED_BOARD,
    }
}

pub fn get_one_of_the_bits(multiple_moves_in_one: u128) -> u128 {
    let mut column = FULL_BOTH_COLOR_LEFT_COLUMN;
    for _ in 1..8 {
        if (multiple_moves_in_one & column).count_ones() > 0 {
            return multiple_moves_in_one & column;
        }
        column *= 4;
    }
    0
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
        );
        assert_eq!(possible_moves(11302567564057283082684841), 3298534883328)
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
            4836883870360778079272960
        );
        assert_eq!(
            compute_winning_positions(11302567564057283082684841, PlayerColor::Blue),
            1099578736640
        );
        assert_eq!(
            compute_winning_positions(11680300390167387516198912, PlayerColor::Red),
            563087392374784
        )
    }

    #[test]
    fn opponent_winning_positions_given_opportunities_return_winning_moves() {
        assert_eq!(
            opponent_winning_positions(75229342058982408192, PlayerColor::Red),
            4836883870360778079272960
        );
        assert_eq!(
            opponent_winning_positions(4613163779234988032, PlayerColor::Red),
            4521191814463488
        );
        assert_eq!(
            opponent_winning_positions(11302567564057283082684841, PlayerColor::Red),
            1099578736640
        )
    }

    #[test]
    fn calculate_non_losing_moves_given_possible_non_loosing_moves_return_those() {
        assert_eq!(
            calculate_non_losing_moves(6825767598171535737952672, PlayerColor::Red),
            8
        );
        assert_eq!(
            calculate_non_losing_moves(11302567564057283082684841, PlayerColor::Red),
            0
        );
        assert_eq!(
            calculate_non_losing_moves(11680300387915587702513664, PlayerColor::Red),
            2814749767239714
        );
        assert_eq!(
            calculate_non_losing_moves(11680300390167387516198912, PlayerColor::Blue),
            0
        )
    }
}
