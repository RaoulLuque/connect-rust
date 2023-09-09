mod bruteforce_helpers;

use std::{collections::VecDeque, thread::current};

use crate::gamestate_helpers::PlayerColor;
use connect_rust_graphs::graph::Graph;

pub struct Engine {
    color: PlayerColor, 
    gamestate_graph: Graph<u32>,
}

impl Engine {
    /// Constructor for new engine constructing empty gamestate graph
    pub fn new(color: PlayerColor) -> Engine {
        let mut res: Engine = Engine{color, gamestate_graph: Graph::new()};
        
        res.initialize_graph();

        res
    }

    /// Returns the best possible move accounting to the gamestate graph calculated at initialization
    pub fn make_move(&self, gamestate: u32) -> u32 {
        0
    }

    /// Initializes the gamestate graph with all possible gamestates
    fn initialize_graph(&mut self) -> () {
        let initial_gamestate: u32 = 0;

        let forhundredforty: usize = 440;

        // Initial evaluation of gamestates is -1
        self.gamestate_graph.add_vertex_with_label(initial_gamestate, "-1");

        let mut unvisited: VecDeque<u32> =  VecDeque::new();
        unvisited.push_back(initial_gamestate);

        while unvisited.len() != 0 {
            let current_gamestate: u32 = unvisited.pop_front().expect("Unvisited Queue should not be empty because of loop invariant");

            // Iterate through possible next gamestates, add edge and possible vertex to graph
            for next_gamestate in bruteforce_helpers::possible_next_gamestates(current_gamestate) {
                // If next gamestate is not in gamestate graph, push to univisted queue
                if !self.gamestate_graph.is_vertex_in_graph(&next_gamestate) {
                    unvisited.push_back(next_gamestate)
                }
                self.gamestate_graph.add_vertex_with_label(next_gamestate, "-1");
                self.gamestate_graph.add_edge(current_gamestate, next_gamestate).expect("Gamestates should be in the gamestate graph");
            }          
        }
    }
}

// to do: Implement tests
#[cfg(test)]
mod tests {
    use crate::players::Player;

    use super::*;
    const BASE: u32 = 2;

    #[test]
    fn initialize_graph_test_for_exemplary_edges_in_gamestate_graph() {
        let e = Engine::new(PlayerColor::Blue);
        assert!(!e.gamestate_graph.is_edge_in_graph(0, 1));
        assert!(e.gamestate_graph.is_edge_in_graph(0, 268435456));
        e.gamestate_graph.number_of_vertices()
    }
}