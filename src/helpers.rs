pub mod encoding_gamestate;
pub mod moves;
pub mod state_of_game;
pub mod whos_turn_is_it;

use serde::Serialize;
use std::collections::VecDeque;

// Const for using pow
const BASE: u128 = 2;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
/// Possible Colors for players, None used if no player has achieved something. Blue starts
/// Blue's moves are encoded  with 2^(14*(row - 1) + 2*(column - 1)) and red's move's with blue's*2
pub enum PlayerColor {
    Red,
    Blue,
}
