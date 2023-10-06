use std::collections::VecDeque;

use crate::gamestate_helpers::{whos_turn_is_it_gamestate, turn_column_to_encoded_gamestate, is_over};

/// Returns the possible next gamestates from a given gamestate as an iterator
pub fn possible_next_gamestates(current_gamestate: u32) -> std::collections::vec_deque::IntoIter<u32> {
    let mut res_queue: VecDeque<u32> = VecDeque::new();
    let player_whos_turn_it_is = whos_turn_is_it_gamestate(current_gamestate);

    // Add possible moves by checking all columns
    for column in 1..5 {
        let next_move = turn_column_to_encoded_gamestate(current_gamestate, column, &player_whos_turn_it_is);
        match next_move {
            Some(i) => {if is_over(i | current_gamestate) {
                res_queue.push_front(i | current_gamestate)
            } else {
                res_queue.push_back(i | current_gamestate);
            }}
            None => (),
        };
    }

    // Return iterator over possible moves
    res_queue.into_iter()
}