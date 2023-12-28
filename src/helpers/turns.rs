use super::*;

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

/// Returns the numbers of turn's played so far
pub fn number_of_turns_played(gamestate: u128) -> u8 {
    gamestate.count_ones() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
