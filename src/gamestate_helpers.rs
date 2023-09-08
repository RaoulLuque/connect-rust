#[derive(Debug)]
#[derive(PartialEq)]
/// Possible Colors for players, None used if no player has achieved something. Blue starts
/// Blue's moves are encoded  with 2^(8*row + 2*column) and red with blue*2
pub enum PlayerColor {
    Red,
    Blue,
}

/// Returns which player color has won the game if so
pub fn is_won(gamestate: u32) -> Option<PlayerColor> {
    // Can't win before 7th turn
    if gamestate.count_ones() < 7 {return None};

    // Vector of possible winning combinations for red
    let red_winning_gamestates: Vec<u32> = vec![85, 21760, 5570560, 1426063360, 16843009, 67372036, 269488144, 1077952576, 1074791425, 17043520];

    //Vector of possible winning combinations for blue
    let blue_winning_gamestates: Vec<u32> = vec![170, 43520, 11141120, 2852126720, 33686018, 134744072, 538976288, 2155905152, 2149582850, 34087040];


    if red_winning_gamestates.into_iter()
        .filter(|x| *x & gamestate == gamestate)
        .collect::<Vec<u32>>()
        .len() > 0  { Some(PlayerColor::Red) }  
    else if blue_winning_gamestates.into_iter()
        .filter(|x| *x & gamestate == gamestate)
        .collect::<Vec<u32>>()
        .len() > 0 { Some(PlayerColor::Blue) }
    else {None}
}

/// Returns whether a move is valid or not e.g. a power of 2 (less than or equal to 2^31 is given with u32)
pub fn is_valid_move(move_to_check: u32) -> bool {
    move_to_check.is_power_of_two()
}

/// Returns wheter a move is allowed with the current state of the game
pub fn is_allowed_move(gamestate: u32, move_to_check: u32, turn_number: usize) -> bool {
    // If move not valid, not allowed either
    if !is_valid_move(move_to_check) {return false;}
    
    // Check if move is of corresponding player
    match whos_turn_is_it(turn_number) {
        // Case where it Red's turn and constant is sum of all powers of 2 with even exponents
        PlayerColor::Red => if move_to_check & 1431655765 != move_to_check {return false;}

        // Case where it Blue's turn and constant is sum of all powers of 2 with odd exponents
        PlayerColor::Blue => if move_to_check & 2863311530 != move_to_check {return false;}
    }

    // If space taken, move not allowed
    if gamestate & move_to_check == move_to_check {return false;}

    // If move at bottom, it is allowed
    let two: u32 = 2;
    if move_to_check >= two.pow(24) {return true;}

    // If move 'above' already done move, it is allowed
    if move_to_check.rotate_right(8) & gamestate == move_to_check.rotate_right(8) {
        return true;
    }

    false
}

/// Returns whether the board is full and the game is over
pub fn is_over(gamestate: u32) -> bool {
    gamestate.count_ones() == 16
}

/// Turns an encoded u32 move into a tuple of numbers from 1 to 4 encoding the position of a move 
/// on the 4x4 connect four grid. The tuple is of the form (row, column)
pub fn move_to_tuple(move_to_transform: u32) -> (u32, u32) {
    // Check which row the move is in
    let mut checker_row: u32 = 0;
    let base: u32 = 2;
    for i in 24..32 {
        checker_row += base.pow(i);
    }
    let mut row = 0;
    let mut column = 0;

    for i in 0..4 {
        if checker_row & move_to_transform == move_to_transform {
            row = 4-i;
            break;
        }
        checker_row /= 256;
    }

    // Check which column the move is in
    let mut checker_column: u32 = 0;
    for i in 0..4 {
        checker_column += base.pow(6+ i*8) + base.pow(7+ i*8);
    }

    for i in 0..4 {
        if checker_column & move_to_transform == move_to_transform {
            column = 4-i;
            break;
        }
        checker_column /= 4;
    }

    (row, column)
}

/// Turns an endoded gamestate into a string that is readable for logging
pub fn encoded_gamestate_to_str (mut gamestate: u32) -> String {
    let mut playing_field: String = "".to_owned();

    // Loop over gamestate encoding and read it from beginning to end with bitshifting
    let mut row = 1;
    for i in 1..17 {
        
        // Distinguish cases of first two bits of gamestate number
        if gamestate & 1 == 1 {
            playing_field.push_str("R");
        } else if gamestate & 2 == 2 {
            playing_field.push_str("B");
        } else {
            playing_field.push_str("O");
        }
        playing_field.push_str(" ");
        gamestate /= 4;

        // New line, if full row has been logged
        if i % 4 == 0 {
            playing_field.push_str(format!(" {} \n", row).as_str());
            row += 1;
        }
    }
    playing_field.push_str("\n");
    playing_field.push_str("1 2 3 4 \n");

    playing_field.push_str("\n");

    playing_field
}

/// Returns who's players turn it is in a string, first turn is turn 1
pub fn whos_turn_is_it (turn_number: usize) -> PlayerColor {
    match turn_number % 2 {
        1 => PlayerColor::Blue,
        0 => PlayerColor::Red,
        _ => PlayerColor::Blue, // case should never be encountered
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const BASE: u32 = 2;

    #[test]
    fn is_won_given_empty_gamestate_return_none() {
        assert_eq!(is_won(0), None);
    }

    #[test]
    fn is_won_given_winning_verticals_red_return_red() {
        assert_eq!(is_won(16843009), Some(PlayerColor::Red));
        assert_eq!(is_won(269488144), Some(PlayerColor::Red));
    }

    #[test]
    fn is_won_given_winning_verticals_blue_return_blue() {
        assert_eq!(is_won(2*16843009), Some(PlayerColor::Blue));
        assert_eq!(is_won(2*269488144), Some(PlayerColor::Blue));
    }

    #[test]
    fn is_won_given_winning_diagonals_red_return_red() {
        assert_eq!(is_won(1074791425), Some(PlayerColor::Red));
        assert_eq!(is_won(17043520), Some(PlayerColor::Red));
    }

    #[test]
    fn is_won_given_winning_diagonals_blue_return_blue() {
        assert_eq!(is_won(2*1074791425), Some(PlayerColor::Blue));
        assert_eq!(is_won(2*17043520), Some(PlayerColor::Blue));
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
        assert_eq!(is_allowed_move(0, BASE.pow(31), 0), true);
        assert_eq!(is_allowed_move(BASE.pow(31), BASE.pow(28), 1), true);
    }

    #[test]
    fn is_allowed_move_given_not_allowed_move_return_false() {
        assert_eq!(is_allowed_move(0, BASE.pow(30), 0), false);
        assert_eq!(is_allowed_move(BASE.pow(31), BASE.pow(29), 1), false);
        assert_eq!(is_allowed_move(BASE.pow(31), BASE.pow(31), 1), false);
        assert_eq!(is_allowed_move(BASE.pow(31) + BASE.pow(28), BASE.pow(31), 2), false);
    }

    #[test]
    fn is_over_given_full_board_return_true() {
        assert_eq!(is_over(1431655765), true);
        assert_eq!(is_over(2863311530), true);
    }

    #[test]
    fn is_over_given_not_full_board_return_false() {
        assert_eq!(is_over(24934), false);
        assert_eq!(is_over(2405), false);
    }

    #[test]
    fn move_to_tuple_given_one_return_one_one_tuple() {
        assert_eq!(move_to_tuple(1), (1,1));
    }

    #[test]
    fn whos_turn_is_it_given_even_return_blue() {
        assert_eq!(whos_turn_is_it(0), PlayerColor::Blue);
        assert_eq!(whos_turn_is_it(100), PlayerColor::Blue);
    }

    #[test]
    fn whos_turn_is_it_given_odd_return_red() {
        assert_eq!(whos_turn_is_it(15), PlayerColor::Red);
        assert_eq!(whos_turn_is_it(1003), PlayerColor::Red);
    }
}