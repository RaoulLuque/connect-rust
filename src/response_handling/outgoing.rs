use super::*;
use crate::helpers::{
    encoding_gamestates::{encoded_gamestate_as_string_for_web, turn_column_to_encoded_gamestate},
    moves::possible_next_gamestates,
    state_of_game::{is_over, is_won},
};
use crate::response_handling::html_template::START_PAGE_TEMPLATE;

use minijinja::render;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GameMoveOutput {
    board_as_string: String,
    current_gamestate_encoded: String,
    computation_time: u128,
    number_of_visited_nodes: u32,
    game_not_over: bool,
    who_won: Option<PlayerColor>,
}

pub async fn start_page() -> Html<String> {
    let r = render!(START_PAGE_TEMPLATE, empty_gamestate_as_string_for_web => encoded_gamestate_as_string_for_web(0, true));
    Html(r)
}

pub fn generate_response(
    current_gamestate: u128,
    column_player_wants_to_play: u32,
    engine_to_play_against: Player,
) -> Html<String> {
    let (new_gamestate, move_was_valid) =
        match calculate_new_gamestate(column_player_wants_to_play, current_gamestate) {
            Some(i) => (i, true),
            None => (
                possible_next_gamestates(current_gamestate).last().unwrap() | current_gamestate,
                false,
            ),
        };

    let response =
        generate_response_gamemoveoutput(new_gamestate, move_was_valid, engine_to_play_against);
    let response = generate_response_string(response);

    Html(response)
}

fn generate_response_string(response: GameMoveOutput) -> String {
    match response.game_not_over {
        true => render!(START_PAGE_TEMPLATE, turn => response),
        false => render!(START_PAGE_TEMPLATE, turn => response, over => true),
    }
}

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

fn generate_response_gamemoveoutput(
    current_gamestate: u128,
    move_was_valid: bool,
    engine_to_play_against: Player,
) -> GameMoveOutput {
    if is_over(current_gamestate) {
        generate_response_for_game_over(current_gamestate, 0, 0, move_was_valid)
    } else {
        let (new_gamestate, _, number_of_visited_nodes, computation_time) =
            engine_to_play_against.make_move(current_gamestate, 0);

        if is_over(new_gamestate) {
            generate_response_for_game_over(
                new_gamestate,
                computation_time,
                number_of_visited_nodes,
                move_was_valid,
            )
        } else {
            generate_response_for_game_not_over(
                new_gamestate,
                computation_time,
                number_of_visited_nodes,
                move_was_valid,
            )
        }
    }
}

fn generate_response_for_game_not_over(
    current_gamestate: u128,
    computation_time: u128,
    number_of_visited_nodes: u32,
    move_was_valid: bool,
) -> GameMoveOutput {
    GameMoveOutput {
        board_as_string: encoded_gamestate_as_string_for_web(current_gamestate, move_was_valid),
        current_gamestate_encoded: format!("{}", current_gamestate),
        computation_time,
        number_of_visited_nodes,
        game_not_over: true,
        who_won: None,
    }
}

// Move was valid in case response is for when user input was last turn
fn generate_response_for_game_over(
    final_gamestate: u128,
    computation_time: u128,
    number_of_visited_nodes: u32,
    move_was_valid: bool,
) -> GameMoveOutput {
    GameMoveOutput {
        board_as_string: encoded_gamestate_as_string_for_web(final_gamestate, move_was_valid),
        current_gamestate_encoded: format!("{}", 0),
        computation_time,
        number_of_visited_nodes,
        game_not_over: false,
        who_won: is_won(final_gamestate),
    }
}
