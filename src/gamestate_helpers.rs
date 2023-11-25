use std::collections::VecDeque;

// Const for using pow
const BASE: u128 = 2;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Possible Colors for players, None used if no player has achieved something. Blue starts
/// Blue's moves are encoded  with 2^(14*(row - 1) + 2*(column - 1)) and red's move's with blue's*2
pub enum PlayerColor {
    Red,
    Blue,
}

/// Returns which player color has won the game if so
pub fn is_won(gamestate: u128) -> Option<PlayerColor> {
    // Can't win before 7th turn
    if gamestate.count_ones() < 6 {
        return None;
    };

    // Vector of possible winning combinations for blue
    let blue_winning_gamestates: Vec<u128> = vec![
        85,
        1392640,
        22817013760,
        373833953443840,
        6124895493223874560,
        100350287760979960791040,
        340,
        5570560,
        91268055040,
        1495335813775360,
        24499581972895498240,
        401401151043919843164160,
        1360,
        22282240,
        365072220160,
        5981343255101440,
        97998327891581992960,
        1605604604175679372656640,
        5440,
        89128960,
        1460288880640,
        23925373020405760,
        391993311566327971840,
        6422418416702717490626560,
        4398314962945,
        17593259851780,
        70373039407120,
        281492157628480,
        1125968630513920,
        4503874522055680,
        18015498088222720,
        72061992352890880,
        288247969411563520,
        1152991877646254080,
        4611967510585016320,
        18447870042340065280,
        73791480169360261120,
        295165920677441044480,
        1180663682709764177920,
        4722654730839056711680,
        18890618923356226846720,
        75562475693424907386880,
        302249902773699629547520,
        1208999611094798518190080,
        4835998444379194072760320,
        281479271743489,
        1125917086973956,
        4503668347895824,
        18014673391583296,
        4611756388245323776,
        18447025552981295104,
        73788102211925180416,
        295152408847700721664,
        75559016665011384745984,
        302236066660045538983936,
        1208944266640182155935744,
        4835777066560728623742976,
        4399120515136,
        17596482060544,
        70385928242176,
        281543712968704,
        72075190519988224,
        288300762079952896,
        1153203048319811584,
        4612812193279246336,
        1180879921479487062016,
        4723519685917948248064,
        18894078743671792992256,
        75576314974687171969024,
    ];

    //Vector of possible winning combinations for red
    let red_winning_gamestates: Vec<u128> = vec![
        170,
        2785280,
        45634027520,
        747667906887680,
        12249790986447749120,
        200700575521959921582080,
        680,
        11141120,
        182536110080,
        2990671627550720,
        48999163945790996480,
        802802302087839686328320,
        2720,
        44564480,
        730144440320,
        11962686510202880,
        195996655783163985920,
        3211209208351358745313280,
        10880,
        178257920,
        2920577761280,
        47850746040811520,
        783986623132655943680,
        12844836833405434981253120,
        8796629925890,
        35186519703560,
        140746078814240,
        562984315256960,
        2251937261027840,
        9007749044111360,
        36030996176445440,
        144123984705781760,
        576495938823127040,
        2305983755292508160,
        9223935021170032640,
        36895740084680130560,
        147582960338720522240,
        590331841354882088960,
        2361327365419528355840,
        9445309461678113423360,
        37781237846712453693440,
        151124951386849814773760,
        604499805547399259095040,
        2417999222189597036380160,
        9671996888758388145520640,
        562958543486978,
        2251834173947912,
        9007336695791648,
        36029346783166592,
        9223512776490647552,
        36894051105962590208,
        147576204423850360832,
        590304817695401443328,
        151118033330022769491968,
        604472133320091077967872,
        2417888533280364311871488,
        9671554133121457247485952,
        8798241030272,
        35192964121088,
        140771856484352,
        563087425937408,
        144150381039976448,
        576601524159905792,
        2306406096639623168,
        9225624386558492672,
        2361759842958974124032,
        9447039371835896496128,
        37788157487343585984512,
        151152629949374343938048,
    ];

    if red_winning_gamestates
        .into_iter()
        .filter(|x| *x & gamestate == *x)
        .collect::<Vec<u128>>()
        .len()
        > 0
    {
        Some(PlayerColor::Red)
    } else if blue_winning_gamestates
        .into_iter()
        .filter(|x| *x & gamestate == *x)
        .collect::<Vec<u128>>()
        .len()
        > 0
    {
        Some(PlayerColor::Blue)
    } else {
        None
    }
}

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

/// Returns whether the board is full and the game is over
pub fn is_full(gamestate: u128) -> bool {
    gamestate.count_ones() == 42
}

/// Returns true if someone has won or the board is full otherwise false
pub fn is_over(gamestate: u128) -> bool {
    if is_full(gamestate) {
        true
    } else {
        match is_won(gamestate) {
            Some(_) => true,
            None => false,
        }
    }
}

/// Turns an encoded u128 move into a tuple of numbers from 1 to 4 encoding the position of a move
/// on the 4x4 connect four grid. The tuple is of the form (row, column)
pub fn move_to_tuple(move_to_transform: u128) -> (u32, u32) {
    let mut row = 0;
    let mut column = 0;

    // Check which row the move is in
    // Checker_row is the encoded first row from the top filled with both blue and red
    let mut checker_row: u128 = 16383;

    for i in 0..6 {
        if checker_row & move_to_transform == move_to_transform {
            row = i + 1;
            break;
        }
        checker_row *= BASE.pow(14);
    }

    // Check which column the move is in
    // Checker_column is the encoded first column from the left filled with both blue and red
    let mut checker_column: u128 = 3541991048129292582915;

    for i in 0..7 {
        if checker_column & move_to_transform == move_to_transform {
            column = i + 1;
            break;
        }
        checker_column *= 4;
    }

    (row, column)
}

/// Turns an encoded gamestate into a string that is readable for logging
pub fn encoded_gamestate_to_str(mut gamestate: u128) -> String {
    let mut playing_field: String = "".to_owned();

    // Loop over gamestate encoding and read it from beginning to end with bit shifting
    let mut row = 1;
    for i in 1..43 {
        // Distinguish cases of first two bits of gamestate number
        if gamestate & 1 == 1 {
            playing_field.push_str("🟦");
        } else if gamestate & 2 == 2 {
            playing_field.push_str("🟥");
        } else {
            playing_field.push_str("🟫");
        }
        playing_field.push_str(" ");
        gamestate /= 4;

        // New line, if full row has been logged
        if i % 7 == 0 {
            playing_field.push_str(format!(" {} \n", row).as_str());
            row += 1;
        }
    }
    playing_field.push_str("\n");
    playing_field.push_str("1  2  3  4  5  6  7 \n");

    playing_field.push_str("\n");

    playing_field
}

/// Returns who's players turn it is in a string based on the current turn numer. First turn is turn 1
pub fn whos_turn_is_it_turn_number(turn_number: usize) -> PlayerColor {
    match turn_number % 2 {
        1 => PlayerColor::Blue,
        0 => PlayerColor::Red,
        _ => PlayerColor::Blue, // case should never be encountered
    }
}

/// Returns who's players turn it is in a string based on the current gamestate. First turn is turn 1
pub fn whos_turn_is_it_gamestate(gamestate: u128) -> PlayerColor {
    whos_turn_is_it_turn_number(
        1 + usize::try_from(gamestate.count_ones())
            .expect("Turn Number should be displayable with 16 Bits"),
    )
}

/// Turns an encoded tuple move into an encoded u128 with the color whos players turn it should be
pub fn turn_column_to_encoded_gamestate(
    gamestate: u128,
    column: u32,
    color: &PlayerColor,
) -> Option<u128> {
    let mut row_counter: u128 = BASE.pow(5 * 14 + (column - 1) * 2);

    let mut int_division = false;
    while (row_counter & gamestate == row_counter)
        || (row_counter * 2 & gamestate == row_counter * 2)
    {
        row_counter /= BASE.pow(14);
        if int_division {
            break;
        }
        if row_counter == 1 || row_counter == 0 {
            int_division = true;
        }
    }
    if row_counter == 0 {
        None
    } else {
        match color {
            PlayerColor::Blue => Some(row_counter),
            PlayerColor::Red => Some(row_counter * 2),
        }
    }
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
            Some(i) => res_queue.push_back(i | current_gamestate),
            None => (),
        };
    }

    // Return iterator over possible moves
    res_queue.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    const BASE: u128 = 2;

    #[test]
    fn is_won_given_empty_gamestate_return_none() {
        assert_eq!(is_won(0), None);
    }

    #[test]
    fn is_won_given_winning_verticals_blue_return_blue() {
        assert_eq!(is_won(39584834666497), Some(PlayerColor::Blue));
        assert_eq!(
            is_won(39584834666497 * 4 * 4 * BASE.pow(28)),
            Some(PlayerColor::Blue)
        );
    }

    #[test]
    fn is_won_given_winning_verticals_red_return_red() {
        assert_eq!(is_won(35191083368472), Some(PlayerColor::Red));
        assert_eq!(
            is_won(35191083368472 * 4 * BASE.pow(14)),
            Some(PlayerColor::Red)
        );
    }

    #[test]
    fn is_won_given_winning_diagonals_blue_return_blue() {
        assert_eq!(is_won(457403279671297), Some(PlayerColor::Blue));
        assert_eq!(
            is_won(457403279671297 * 4 * 4 * BASE.pow(14)),
            Some(PlayerColor::Blue)
        );
    }

    #[test]
    fn is_won_given_winning_diagonals_red_return_red() {
        assert_eq!(is_won(8798509547648), Some(PlayerColor::Red));
        assert_eq!(
            is_won(8798509547648 * 4 * 4 * BASE.pow(28)),
            Some(PlayerColor::Red)
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
    fn is_over_given_someone_has_won_return_true() {
        assert_eq!(is_over(688213), true);
    }

    #[test]
    fn is_over_given_full_board_return_true() {
        assert_eq!(is_over(6447604371278022265099605), true);
    }

    #[test]
    fn is_full_given_full_board_return_true() {
        assert_eq!(is_full(6447604371278022265099605), true);
    }

    #[test]
    fn is_full_given_not_full_board_return_false() {
        assert_eq!(is_full(24934), false);
        assert_eq!(is_full(2405), false);
    }

    #[test]
    fn move_to_tuple_given_one_return_one_one_tuple() {
        assert_eq!(move_to_tuple(1), (1, 1));
    }

    #[test]
    fn encoded_gamestate_to_str_given_gamestate_return_wanted_string() {
        assert_eq!(encoded_gamestate_to_str(0), "🟫 🟫 🟫 🟫 🟫 🟫 🟫  1 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  2 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  3 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  4 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  5 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  6 \n\n1  2  3  4  5  6  7 \n\n");
    }

    #[test]
    fn encoded_gamestate_to_str_given_gamestates_return_wanted_colors() {
        let gamestate = 1;
        let gamestate_str = encoded_gamestate_to_str(gamestate);
        assert_eq!(&gamestate_str[..4], "🟦");

        let gamestate_str = encoded_gamestate_to_str(2);
        assert_eq!(&gamestate_str[..4], "🟥");

        let gamestate_str = encoded_gamestate_to_str(2147483648);
        assert_eq!(&gamestate_str[30..34], "🟫");
    }

    #[test]
    fn whos_turn_is_it_given_even_return_red() {
        assert_eq!(whos_turn_is_it_turn_number(0), PlayerColor::Red);
        assert_eq!(whos_turn_is_it_turn_number(100), PlayerColor::Red);
    }

    #[test]
    fn whos_turn_is_it_given_odd_return_blue() {
        assert_eq!(whos_turn_is_it_turn_number(15), PlayerColor::Blue);
        assert_eq!(whos_turn_is_it_turn_number(1003), PlayerColor::Blue);
    }

    #[test]
    fn turn_column_to_encoded_gamestate_given_correct_tuples_blue_return_encoded_move() {
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 7, &PlayerColor::Blue),
            Some(BASE.pow(82))
        );
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 6, &PlayerColor::Blue),
            Some(BASE.pow(80))
        );
    }

    #[test]
    fn turn_column_to_encoded_gamestate_given_correct_tuples_red_return_encoded_move() {
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 6, &PlayerColor::Red),
            Some(BASE.pow(81))
        );
    }

    #[test]
    fn possible_next_gamestates_given_gamestate_return_next_gamestate_is_in_iterator() {
        let vec: Vec<u128> = possible_next_gamestates(BASE.pow(82)).collect();
        assert!(vec.contains(&(BASE.pow(81) + BASE.pow(82))));
    }
}
