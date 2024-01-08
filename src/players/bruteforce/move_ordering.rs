use crate::helpers::{moves::compute_winning_positions, PlayerColor};

pub fn move_score(gamestate: u128, color: PlayerColor) -> u8 {
    compute_winning_positions(gamestate, color).count_ones() as u8
}
