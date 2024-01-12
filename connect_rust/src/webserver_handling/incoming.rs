use super::*;

use axum::extract::Form;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Struct for receiving data from the user.
/// - current_and_previous_gamestates are the current and previous gamestates in a string separated
/// by "#" in column encoding (gamestate encoding (2)). Starting with the current ending with the
/// first gamestate, e.g. current_gamestate#...#first_gamestate
/// - column is the index of the column the user wishes to play in
/// - engine is the name of the engine the user wants to play against
#[derive(Debug, Deserialize)]
pub struct GameMoveInput {
    current_and_previous_gamestates: Option<String>,
    #[serde(deserialize_with = "deserialize_integer_or_zero")]
    column: u32,
    engine: Option<String>,
}

/// Accepts the html form in scheme of the start page html template and returns a response in the same scheme. 
/// "/" is routed to this function.
pub async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let (column_player_wants_to_play, current_and_previous_gamestates, engine_to_play_against) =
        read_in_response(turn);

    outgoing::generate_response(
        current_and_previous_gamestates,
        column_player_wants_to_play,
        engine_to_play_against,
    )
}

/// Given a game move input, returns the a three tuple with:
/// - The column the user wants to play in
/// - The current and previous gamestates encoded as a string separated
/// by "#" in column encoding (gamestate encoding (2)). Starting with the current ending with the
/// first gamestate, e.g. current_gamestate#...#first_gamestate
/// - The engine the user wants to play against as an player enum variant
fn read_in_response(turn: GameMoveInput) -> (u32, String, Player) {
    match (
        turn.column,
        turn.current_and_previous_gamestates,
        turn.engine,
    ) {
        (column_as_integer, Some(current_and_previous_gamestates), Some(engine_as_string)) => (
            column_as_integer,
            current_and_previous_gamestates,
            Player::from_str(&engine_as_string).expect("From str should always return ok"),
        ),
        // (_, Some(current_and_previous_gamestates), Some(engine_as_string)) => (
        //     0,
        //     current_and_previous_gamestates,
        //     Player::from_str(&engine_as_string).expect("From str should always return ok"),
        // ),
        (_, Some(current_and_previous_gamestates), _) => {
            (0, current_and_previous_gamestates, Player::Random)
        }
        _ => (0, "".to_owned(), Player::Random),
    }
}

/// Deserializes an integer to itself or 0 if deserialization fails
fn deserialize_integer_or_zero<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized = u32::deserialize(deserializer);
    // deserialized is a Result<bool, ...>. You can handle the error
    // case however you like. This example simply returns false.
    Ok(deserialized.unwrap_or(0))
}
