use crate::helpers::moves::possible_next_gamestates;
pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i8, u32, u128) {
        (
            possible_next_gamestates(current_gamestate).last().unwrap(),
            0,
            0,
            0,
        )
    }
}
