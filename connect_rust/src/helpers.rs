/// Helper module with functions for move handling, checking state of the game and encoding and decoding gamestates

pub mod encoding_gamestates;
pub mod moves;
pub mod state_of_game;
pub mod turns;

use serde::Serialize;
use std::collections::VecDeque;

// Const for using pow
const BASE: u128 = 2;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
/// Possible Colors for players, None used if no player has achieved something. Blue starts
pub enum PlayerColor {
    Red,
    Blue,
}
