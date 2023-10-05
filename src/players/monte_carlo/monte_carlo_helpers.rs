use std::time::Instant;

use crate::gamestate_helpers::{possible_next_gamestates, is_over, PlayerColor, PlayerColor::*};
use super::Engine;

// Monte Carlo Selection coefficient
const C: f64 = 0.7;


impl Engine {
    // Time is passed as milliseconds
    pub fn make_move(&mut self, gamestate: u32, time: u128) -> u32 {
        let end_time = Instant::now().elapsed().as_millis() + time;
        while end_time > Instant::now().elapsed().as_millis() {
            let mut current_node = gamestate;
            let mut last_node = current_node;

            // Selecting until leaf is found
            // Selected node is not in gamestate graph
            while self.gamestate_graph.is_vertex_in_graph(&current_node) {
                last_node = current_node;
                match self.selection(current_node) {
                    None => {break;},
                    Some(node) => {current_node = node;},
                }
            }

            if is_over(current_node) {
                continue;
            }

            // Add new node (current node) to the gamestate graph
            self.expand(last_node, current_node);
            let rating: Option<PlayerColor> = Engine::simulate_game(current_node);

            loop {
                self.backpropagate(current_node, rating);

                if current_node == 0 {
                    break;
                }
                current_node = *self .gamestate_graph
                                    .in_neighbours(&current_node)
                                    .next()
                                    .expect("Gamestate should have parent");
            }
        }

        0
    }

    /// Selects one of the children of a given node
    /// Uses the UCT (Upper confidence bound applied to trees)
    fn selection (&self, current_gamestate: u32) -> Option<u32> {
        // If one of the children is not in gamestate graph, it is selected
        for successor in possible_next_gamestates(current_gamestate) {
            if !self.gamestate_graph.is_vertex_in_graph(&successor) {
                return Some(successor);
            }
        }

        // Check whether gamestate has been visited less than 30 times
        if self .gamestate_evaluations
                .get(&current_gamestate)
                .expect("Gamestate should be in gamestate evaluations")
                .1 < 30 {
            return Some(self.simulation_picker(current_gamestate));
    
        } else {
            // By case all of the children are in the gamestate graph
            let mut best_successor = 0;
            for successor in possible_next_gamestates(current_gamestate) {
                if best_successor == 0 {
                    best_successor = successor;
                } else {
                    // Compare evaluation formula
                    if (self.gamestate_evaluations.get(&successor)
                        .expect("Successor should be in gamestate evaluations")
                        .0 as f64 / 
                    self.gamestate_evaluations.get(&successor)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64
                    + C * ((self.gamestate_evaluations.get(&current_gamestate)
                    .expect("Successor should be in gamestate evaluations")
                    .1 as f64).ln() / 
                    (self.gamestate_evaluations.get(&successor)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64)
                    )) >= 
                    (self.gamestate_evaluations.get(&best_successor)
                        .expect("Successor should be in gamestate evaluations")
                        .0 as f64 / 
                    self.gamestate_evaluations.get(&best_successor)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64
                    + C * ((self.gamestate_evaluations.get(&current_gamestate)
                    .expect("Successor should be in gamestate evaluations")
                    .1 as f64).ln() / 
                    (self.gamestate_evaluations.get(&best_successor)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64)
                    )) {
                        best_successor = successor;
                    }
                }
            }  
            if best_successor != 0 {
                return Some(best_successor)
            } else {
                return None
            }
        }
    }

    fn expand(&self, last_node: u32, current_node: u32) {

    }

    fn simulate_game(starting_node: u32) -> Option<PlayerColor> {
        None
    }

    // Propagate the rating of the simulated game to the parent nodes
    // -1 is added to the rating of each node if the simulated game was lost
    // 1 if won and 0 if it was a draw
    fn backpropagate(&mut self, node: u32, rating: Option<PlayerColor>) {
        let rating = match rating {
            None => 0,
            Some(Blue) => match self.color {Blue => 1, Red => -1},
            Some(Red) => match self.color {Blue => -1, Red => 1},
        };

        let mut eval = self.gamestate_evaluations.entry(node).or_insert((0,1));
        eval.0 += rating;
        eval.1 += 1;
        
    }

    fn simulation_picker (&self, current_node: u32) -> u32 {
        0
    }
}