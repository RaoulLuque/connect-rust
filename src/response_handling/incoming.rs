use super::*;

use axum::extract::Form;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct GameMoveInput {
    // Gamestates separated by '#' from current to last gamestate in column encoding
    current_and_previous_gamestates: Option<String>,
    #[serde(deserialize_with = "deserialize_integer_or_zero")]
    column: u32,
    engine: Option<String>,
}

pub async fn accept_move(Form(turn): Form<GameMoveInput>) -> Html<String> {
    let (column_player_wants_to_play, current_and_previous_gamestates, engine_to_play_against) =
        read_in_response(turn);

    outgoing::generate_response(
        current_and_previous_gamestates,
        column_player_wants_to_play,
        engine_to_play_against,
    )
}

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

fn deserialize_integer_or_zero<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized = u32::deserialize(deserializer);
    // deserialized is a Result<bool, ...>. You can handle the error
    // case however you like. This example simply returns false.
    Ok(deserialized.unwrap_or(0))
}
