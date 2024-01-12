use std::ops::BitXor;

use super::*;
use crate::helpers::{
    encoding_gamestates::{
        encoded_gamestate_as_string_for_web, encoded_gamestate_to_column,
        turn_column_to_encoded_gamestate, turn_series_of_columns_to_encoded_gamestate,
    },
    state_of_game::{is_over, is_won},
};
use crate::players::bruteforce_n_percent::PossiblePercentages;
use crate::players::random;
use crate::webserver_handling::how_to_play_html_template::HOW_TO_PLAY_TEMPLATE;
use crate::webserver_handling::start_page_html_template::START_PAGE_TEMPLATE;

use minijinja::render;
use serde::Serialize;

/// Struct for sending a response to the webserver using minijinja templating.
/// - boards is a vector of the previous gamestates as strings in column encoding (gamestate encoding (2)).
/// Starts with the current gamestates and ends with the first gamestate
/// - final_board_as_string is the string of the current board and used for displaying the final gamestate if game is over
/// - computation_time is the time it took the user chosen engine to compute the move it made
/// - number_of_visited node is the number of nodes visited by the engine whilst computing the move
/// it made.
/// - game_not_over is true if the game is still going and false if it is over either by the user's
/// or the engine's move
/// - who_won is Some(PlayerColor) with the color of the player who won if the game is won by someone
/// and otherwise None
/// - move_was_valid is true if the column the user inputted was valid and otherwise false
#[derive(Debug, Serialize)]
pub struct GameMoveOutput {
    boards: Vec<String>,
    final_board_as_string: String,
    computation_time: u128,
    number_of_visited_nodes: u32,
    game_not_over: bool,
    who_won: Option<PlayerColor>,
    move_was_invalid: bool,
}

/// Statically serves the start_page template with the empty board
pub async fn start_page() -> Html<String> {
    let r = render!(START_PAGE_TEMPLATE, empty_gamestate_as_string_for_web => encoded_gamestate_as_string_for_web(0));
    Html(r)
}

/// Statically serves the how_to_play page
pub async fn how_to_play_page() -> Html<String> {
    let r = render!(HOW_TO_PLAY_TEMPLATE);
    Html(r)
}

/// Generates the Html response given the necessary information by the user.
pub fn generate_response(
    current_and_previous_gamestates: String,
    mut column_player_wants_to_play: u32,
    engine_to_play_against: Player,
) -> Html<String> {
    let mut last_gamestate_and_those_before: Vec<String> =
        match current_and_previous_gamestates.trim().len() {
            0 => vec!["".to_string()],
            _ => current_and_previous_gamestates
                .split('#')
                .map(|s| s.trim().to_string())
                .collect(),
        };

    let previous_gamestate = turn_series_of_columns_to_encoded_gamestate(
        last_gamestate_and_those_before.get(0).unwrap(),
    );

    let (new_gamestate, move_was_valid) =
        match calculate_new_gamestate(column_player_wants_to_play, previous_gamestate) {
            Some(i) => (i, true),
            None => {
                // Move wasn't valid and column_player_wants_play must be overwritten with random column
                let next_move = random::Engine::make_move(previous_gamestate).0;
                column_player_wants_to_play =
                    encoded_gamestate_to_column(next_move.bitxor(previous_gamestate))
                        .expect("Next move shouldn't be empty");
                (next_move | previous_gamestate, false)
            }
        };

    // Insert the new gamestate into the vector with all gamestates
    last_gamestate_and_those_before.insert(
        0,
        format!(
            "{}{}",
            last_gamestate_and_those_before
                .get(0)
                .expect("Vector with last gamestates and those before shouldn't be empty"),
            column_player_wants_to_play
        ),
    );

    let response = generate_response_gamemoveoutput(
        new_gamestate,
        &mut last_gamestate_and_those_before,
        move_was_valid,
        &engine_to_play_against,
    );

    let response = generate_response_string(response, &engine_to_play_against);

    Html(response)
}

/// Generates the response string by rendering the template as a string with minijinja given the
/// response and the engine the player choose in order to auto select said engine for convenience
fn generate_response_string(response: GameMoveOutput, engine_to_play_against: &Player) -> String {
    match (response.game_not_over, engine_to_play_against) {
        (true, Player::Bruteforce) => {
            render!(START_PAGE_TEMPLATE, turn => response, bruteforce => true)
        }
        (true, Player::BruteforceNPercent(PossiblePercentages::TwentyFive)) => {
            render!(START_PAGE_TEMPLATE, turn => response, bruteforce_twenty_five_percent => true)
        }
        (true, Player::BruteforceNPercent(PossiblePercentages::Fifty)) => {
            render!(START_PAGE_TEMPLATE, turn => response, bruteforce_fifty_percent => true)
        }
        (true, Player::BruteforceNPercent(PossiblePercentages::SeventyFive)) => {
            render!(START_PAGE_TEMPLATE, turn => response, bruteforce_seventy_five_percent => true)
        }
        (true, Player::MonteCarlo) => {
            render!(START_PAGE_TEMPLATE, turn => response, monte_carlo => true)
        }
        (true, Player::Random) => render!(START_PAGE_TEMPLATE, turn => response, random => true),
        (true, Player::RandomGlowedUp) => {
            render!(START_PAGE_TEMPLATE, turn => response, random_glowed_up => true)
        }
        (false, Player::Bruteforce) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, bruteforce => true)
        }
        (false, Player::BruteforceNPercent(PossiblePercentages::TwentyFive)) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, bruteforce_twenty_five_percent => true)
        }
        (false, Player::BruteforceNPercent(PossiblePercentages::Fifty)) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, bruteforce_fifty_percent => true)
        }
        (false, Player::BruteforceNPercent(PossiblePercentages::SeventyFive)) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, bruteforce_seventy_five_percent => true)
        }
        (false, Player::MonteCarlo) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, monte_carlo => true)
        }
        (false, Player::Random) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, random => true)
        }
        (false, Player::RandomGlowedUp) => {
            render!(START_PAGE_TEMPLATE, turn => response, over => true, random_glowed_up => true)
        }
    }
}

/// Returns the new gamestate if the column the player wants to play is valid. Otherwise returns
/// None
pub fn calculate_new_gamestate(
    column_player_wants_to_play: u32,
    current_gamestate: u128,
) -> Option<u128> {
    match turn_column_to_encoded_gamestate(
        current_gamestate,
        column_player_wants_to_play,
        &PlayerColor::Blue,
    ) {
        Some((i, _)) => Some(i | current_gamestate),
        // Possible_next_gamestates should not be empty at this point
        None => None,
    }
}

/// Returns GameMoveOutput for constructing the response given the current_gamestate,
/// the current and previous gamestates, whether the move was valid and which engine the user wants
/// to play against.
fn generate_response_gamemoveoutput(
    current_gamestate: u128,
    current_gamestate_and_those_before: &mut Vec<String>,
    move_was_valid: bool,
    engine_to_play_against: &Player,
) -> GameMoveOutput {
    if is_over(current_gamestate) {
        generate_response_based_on_game_over(
            current_gamestate,
            current_gamestate_and_those_before,
            0,
            0,
            move_was_valid,
            true,
        )
    } else {
        let (new_gamestate, _, number_of_visited_nodes, computation_time) =
            engine_to_play_against.make_move(current_gamestate);

        let column_engine_wants_to_play =
            encoded_gamestate_to_column(new_gamestate.bitxor(current_gamestate))
                .expect("Engine should make move since game is not over");

        // Insert the new gamestate into the vector with all gamestates
        current_gamestate_and_those_before.insert(
            0,
            format!(
                "{}{}",
                current_gamestate_and_those_before
                    .get(0)
                    .expect("Vector with last gamestates and those before shouldn't be empty"),
                column_engine_wants_to_play
            ),
        );

        generate_response_based_on_game_over(
            new_gamestate,
            current_gamestate_and_those_before,
            computation_time,
            number_of_visited_nodes,
            move_was_valid,
            is_over(new_gamestate),
        )
    }
}

/// Helper function for [generate_response_gamemoveoutput].
/// Returns GameMoveOutput for constructing response considering whether the game is over.
fn generate_response_based_on_game_over(
    new_gamestate: u128,
    current_gamestate_and_those_before_as_vector: &mut Vec<String>,
    computation_time: u128,
    number_of_visited_nodes: u32,
    move_was_valid: bool,
    game_over: bool,
) -> GameMoveOutput {
    let current_gamestate_and_those_before_as_vector =
        turn_vector_of_strings_of_columns_to_vector_of_encoded_gamestates(
            current_gamestate_and_those_before_as_vector,
        );

    match game_over {
        false => GameMoveOutput {
            boards: current_gamestate_and_those_before_as_vector,
            final_board_as_string: "".to_string(),
            computation_time,
            number_of_visited_nodes,
            game_not_over: true,
            who_won: None,
            move_was_invalid: !move_was_valid,
        },
        true => {
            let final_board_as_string = current_gamestate_and_those_before_as_vector
                .get(0)
                .expect("current_gamestate_and_those_before_as_vector shouldn't be empty")
                .to_string();
            GameMoveOutput {
                boards: current_gamestate_and_those_before_as_vector,
                final_board_as_string,
                computation_time,
                number_of_visited_nodes,
                game_not_over: false,
                who_won: is_won(new_gamestate),
                move_was_invalid: !move_was_valid,
            }
        }
    }
}

/// Returns a vector with gamestates in u128 gamestate encoding (1) given a vector with gamestates in
/// column gamestate encoding (2)
fn turn_vector_of_strings_of_columns_to_vector_of_encoded_gamestates(
    vector: &Vec<String>,
) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for gamestate_as_columns in vector {
        res.push(encoded_gamestate_as_string_for_web(
            turn_series_of_columns_to_encoded_gamestate(&gamestate_as_columns),
        ));
    }
    res
}
