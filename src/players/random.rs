use crate::gamestate_helpers::possible_next_gamestates;
pub struct Engine;

impl Engine {
    pub fn make_move(current_gamestate: u128) -> (u128, i32, u32, u32) {
        (
            possible_next_gamestates(current_gamestate).last().unwrap(),
            0,
            0,
            0,
        )
    }
}
