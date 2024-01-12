use crate::helpers::{moves::compute_winning_positions, PlayerColor};

/// Returns the move score given an encoded gamestate and the player who wants the score by computing
/// the number of "three in a rows" (rows, columns or diagonals).
pub fn move_score(gamestate: u128, color: PlayerColor) -> u8 {
    compute_winning_positions(gamestate, color).count_ones() as u8
}
