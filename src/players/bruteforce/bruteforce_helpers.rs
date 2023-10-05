use std::collections::VecDeque;

use crate::gamestate_helpers::{whos_turn_is_it_gamestate, turn_column_to_encoded_gamestate};

/// Returns the possible next gamestates from a given gamestate as an iterator
/// To do: Sort Gamestates 
pub fn possible_next_gamestates(current_gamestate: u32) -> std::collections::vec_deque::IntoIter<u32> {
    let mut res_queue: VecDeque<u32> = VecDeque::new();
    let player_whos_turn_it_is = whos_turn_is_it_gamestate(current_gamestate);

    // Add possible moves by checking all columns
    for column in 1..5 {
        let next_move = turn_column_to_encoded_gamestate(current_gamestate, column, &player_whos_turn_it_is);
        match next_move {
            Some(i) => res_queue.push_back(i | current_gamestate),
            None => (),
        };
    }

    // Return iterator over possible moves
    res_queue.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_next_gamestates_given_gamestate_return_next_gamestate_is_in_iterator() {
        let vec: Vec<u32> = possible_next_gamestates(2779152705).collect();
        assert!(vec.contains(&2779156801));
    }
}