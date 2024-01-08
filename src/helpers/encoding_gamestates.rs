use super::moves::FULL_BOTH_COLOR_LEFT_COLUMN;
use super::*;

use std::ops::BitOr;

/// Turns an encoded gamestate into a string that is readable for logging
pub fn encoded_gamestate_to_str(mut gamestate: u128, line_break_str: &str) -> String {
    let mut playing_field: String = "".to_owned();

    if line_break_str == "\n" {
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
                playing_field.push_str(format!(" {} {}", row, line_break_str).as_str());
                row += 1;
            }
        }
        playing_field.push_str(line_break_str);
        for i in 1..8 {
            playing_field.push_str(i.to_string().as_str());
            playing_field.push_str("  ");
        }
    } else if line_break_str == "<br>" {
        // Loop over gamestate encoding and read it from beginning to end with bit shifting
        for _ in 1..43 {
            // Distinguish cases of first two bits of gamestate number
            if gamestate & 1 == 1 {
                playing_field.push_str(r#"<div class="box_blue"></div>"#);
            } else if gamestate & 2 == 2 {
                playing_field.push_str(r#"<div class="box_red"></div>"#);
            } else {
                playing_field.push_str(r#"<div class="box_white"></div>"#);
            }
            gamestate /= 4;
        }
    }

    playing_field.push_str(line_break_str);
    playing_field.push_str(line_break_str);

    playing_field
}

/// Turns an encoded tuple move into an encoded u128 (!move!) with the color whos players turn it should be
/// Furthermore returns the number of the row the token was placed starting counting at 1
pub fn turn_column_to_encoded_gamestate(
    gamestate: u128,
    column: u32,
    color: &PlayerColor,
) -> Option<(u128, u8)> {
    // Initialize as lowest possible token in given column
    let mut row_counter: u128 = BASE.pow(5 * 14 + (column - 1) * 2);
    let mut row_number: u8 = 1;

    let mut int_division = false;
    while (row_counter & gamestate == row_counter)
        || (row_counter * 2 & gamestate == row_counter * 2)
    {
        row_counter /= BASE.pow(14);
        row_number += 1;

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
            PlayerColor::Blue => Some((row_counter, row_number)),
            PlayerColor::Red => Some((row_counter * 2, row_number)),
        }
    }
}

/// Returns the encoded gamestate as a string with an encoding suitable for web/html
pub fn encoded_gamestate_as_string_for_web(gamestate: u128) -> String {
    encoded_gamestate_to_str(gamestate, "<br>")
}

pub fn turn_series_of_columns_to_encoded_gamestate(series_of_columns: &str) -> u128 {
    if (series_of_columns.trim().len()) == 0 {
        return 0;
    }

    let mut current_player = PlayerColor::Blue;
    let mut current_gamestate = 0;

    for char in series_of_columns.chars() {
        current_gamestate = turn_column_to_encoded_gamestate(
            current_gamestate,
            char.to_digit(10)
                .expect("Character in string should be a number"),
            &current_player,
        )
        .expect("Move should be possible")
        .0
        .bitor(current_gamestate);

        current_player = match current_player {
            PlayerColor::Blue => PlayerColor::Red,
            PlayerColor::Red => PlayerColor::Blue,
        };
    }

    current_gamestate
}

/// Given an encoded gamestate returns the first column for which there is a token in from left
/// to right, if there is such a column otherwise None.
pub fn encoded_gamestate_to_column(gamestate: u128) -> Option<u32> {
    let mut column_encoded = FULL_BOTH_COLOR_LEFT_COLUMN;
    for column in 1..8 {
        if column_encoded & gamestate > 0 {
            return Some(column);
        }
        column_encoded *= 4;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_column_to_encoded_gamestate_given_correct_tuples_blue_return_encoded_move() {
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 7, &PlayerColor::Blue),
            Some((BASE.pow(82), 1))
        );
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 6, &PlayerColor::Blue),
            Some((BASE.pow(80), 1))
        );

        assert_eq!(
            turn_column_to_encoded_gamestate(1813388729421943762059264, 6, &PlayerColor::Blue),
            Some((73786976294838206464, 2))
        );
        assert_eq!(
            turn_column_to_encoded_gamestate(1813499416641785460424704, 6, &PlayerColor::Blue),
            Some((274877906944, 4))
        );
    }

    #[test]
    fn turn_column_to_encoded_gamestate_given_correct_tuples_red_return_encoded_move() {
        assert_eq!(
            turn_column_to_encoded_gamestate(0, 6, &PlayerColor::Red),
            Some((BASE.pow(81), 1))
        );
    }

    #[test]
    fn encoded_gamestate_to_str_given_gamestate_return_wanted_string() {
        assert_eq!(encoded_gamestate_to_str(0, "\n"), "🟫 🟫 🟫 🟫 🟫 🟫 🟫  1 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  2 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  3 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  4 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  5 \n🟫 🟫 🟫 🟫 🟫 🟫 🟫  6 \n\n1  2  3  4  5  6  7  \n\n");
    }

    #[test]
    fn encoded_gamestate_to_str_given_gamestates_return_wanted_colors() {
        let gamestate = 1;
        let gamestate_str = encoded_gamestate_to_str(gamestate, "\n");
        assert_eq!(&gamestate_str[..4], "🟦");

        let gamestate_str = encoded_gamestate_to_str(2, "\n");
        assert_eq!(&gamestate_str[..4], "🟥");

        let gamestate_str = encoded_gamestate_to_str(2147483648, "\n");
        assert_eq!(&gamestate_str[30..34], "🟫");
    }

    #[test]
    fn encoded_gamestate_as_string_for_web_given_empty_gamestate_return_correct_string() {
        assert_eq!(encoded_gamestate_as_string_for_web(0), "<div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><div class=\"box_white\"></div><br><br>");
    }
}
