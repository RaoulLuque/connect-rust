use super::{
    encoding_gamestates::turn_column_to_encoded_gamestate,
    turns::{whos_turn_is_it_gamestate, whos_turn_is_it_turn_number},
    *,
};

use std::ops::BitXor;

/// Returns whether a move is valid or not e.g. a power of 2 (less than or equal to 2^31 is given with u128)
pub fn is_valid_move(move_to_check: u128) -> bool {
    move_to_check.is_power_of_two()
}

/// Returns whether a move is allowed with the current state of the game
pub fn is_allowed_move(gamestate: u128, move_to_check: u128, turn_number: usize) -> bool {
    // If move not valid, not allowed either
    if !is_valid_move(move_to_check) {
        return false;
    }

    // Check if move is of corresponding player
    match whos_turn_is_it_turn_number(turn_number) {
        // Case where it Blue's turn and constant is sum of all powers of 2 with even exponents
        PlayerColor::Blue => {
            if move_to_check & (6447604371278022265099605) != move_to_check {
                return false;
            }
        }

        // Case where it Red's turn and constant is sum of all powers of 2 with odd exponents
        PlayerColor::Red => {
            if move_to_check & 12895208742556044530199210 != move_to_check {
                return false;
            }
        }
    }

    // If space taken, move not allowed
    if gamestate & move_to_check == move_to_check {
        return false;
    }

    // If move at bottom, it is allowed
    if move_to_check >= BASE.pow(70) {
        return true;
    }

    // If move 'above' already done move, it is allowed
    match whos_turn_is_it_turn_number(turn_number) {
        // Possible that already done move is from other color
        PlayerColor::Blue => {
            if move_to_check.rotate_left(14) & gamestate == move_to_check.rotate_left(14)
                || move_to_check.rotate_left(15) & gamestate == move_to_check.rotate_left(15)
            {
                return true;
            }
        }
        PlayerColor::Red => {
            if (move_to_check.rotate_left(14) & gamestate == move_to_check.rotate_left(14))
                || (move_to_check.rotate_left(13) & gamestate == move_to_check.rotate_left(13))
            {
                return true;
            }
        }
    }

    false
}

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
    let up_bound: u8 = 6 - row_of_move;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_next_gamestates_given_gamestate_return_next_gamestate_is_in_iterator() {
        let vec: Vec<u128> = possible_next_gamestates(BASE.pow(82)).collect();
        assert!(vec.contains(&(BASE.pow(81) + BASE.pow(82))));
    }

    #[test]
    fn is_allowed_move_given_not_allowed_move_return_false() {
        assert_eq!(is_allowed_move(0, BASE.pow(30), 1), false);
        assert_eq!(is_allowed_move(BASE.pow(31), BASE.pow(29), 2), false);
        assert_eq!(is_allowed_move(BASE.pow(31), BASE.pow(31), 2), false);
        assert_eq!(
            is_allowed_move(BASE.pow(31) + BASE.pow(28), BASE.pow(31), 3),
            false
        );
    }

    #[test]
    fn is_valid_move_given_power_of_two_return_true() {
        assert_eq!(is_valid_move(BASE.pow(10)), true);
        assert_eq!(is_valid_move(BASE.pow(30)), true);
    }

    #[test]
    fn is_valid_move_given_not_power_of_two_return_false() {
        assert_eq!(is_valid_move(7), false);
        assert_eq!(is_valid_move(3), false);
    }

    #[test]
    fn is_allowed_move_given_allowed_move_return_true() {
        assert_eq!(is_allowed_move(0, BASE.pow(70), 1), true);
        assert_eq!(is_allowed_move(BASE.pow(70), BASE.pow(57), 2), true);
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
}
