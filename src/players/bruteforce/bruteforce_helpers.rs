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

pub fn mirror_gamestate(gamestate_to_be_mirrored: u32) -> u32 {
    // Move first colum to fourth
    (gamestate_to_be_mirrored & 50529027) * 64 +
    // Move second column to third
    (gamestate_to_be_mirrored & 202116108) * 4 +
    // Move third to second colum
    (gamestate_to_be_mirrored & 808464432) / 4 +
    // Move fourth column to first
    (gamestate_to_be_mirrored & 3233857728) / 64
}