use std::collections::VecDeque;

use crate::gamestate_helpers::{whos_turn_is_it_gamestate, turn_column_to_encoded_gamestate, is_over};

/// Returns the possible next gamestates from a given gamestate as an iterator
pub fn possible_next_gamestates(current_gamestate: u128) -> std::collections::vec_deque::IntoIter<u128> {
    let mut res_queue: VecDeque<u128> = VecDeque::new();
    let player_whos_turn_it_is = whos_turn_is_it_gamestate(current_gamestate);

    // Add possible moves by checking all columns
    for column in 1..8 {
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

pub fn mirror_gamestate(gamestate_to_be_mirrored: u128) -> u128 {
    // Move first colum to seventh
    (3541991048129292582915 & gamestate_to_be_mirrored) * 4096 +
    // Move second colum to sixth
    (14167964192517170331660 & gamestate_to_be_mirrored) * 256 +
    // Move third colum to fifth
    (56671856770068681326640 & gamestate_to_be_mirrored) * 16 +

    // Move fifth colum to third
    (906749708321098901226240 & gamestate_to_be_mirrored) / 16 +
    // Move sixth colum to second
    (3626998833284395604904960 & gamestate_to_be_mirrored) / 256 +
    // Move seventh colum to first
    (14507995333137582419619840 & gamestate_to_be_mirrored) / 4096
}