mod bruteforce_helpers;

use std::ops::BitXor;

use crate::gamestate_helpers::{PlayerColor, whos_turn_is_it_gamestate, is_over, is_won, possible_next_gamestates};
use connect_rust_graphs::graph::Graph;

pub struct Engine {
    color: PlayerColor, 
    // Gamestates graph where gamestates have labels with their evaluation as i32.
    // "" is the initial evaluation
    // Evaluation +i32::max-x stands for the blue player winning in x turns 
    // Evaluation -i32::max+x stands for the red player winning in x turns
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
        match self.color {
            PlayerColor::Blue => {
                // println!("Rating of position blue can win: {}", self.gamestate_graph.get_label(&2779152705).unwrap());
                // println!("Rating of position blue wins: {}", self.gamestate_graph.get_label(&2779156801).unwrap());

                let mut best_successor: u32 = 0;
                for successor in self.gamestate_graph.out_neighbours(&gamestate) {
                    if best_successor == 0 {
                        best_successor = *successor;
                    }
                    if let (Ok(s), Ok(b)) = (self.gamestate_graph.get_label(successor).unwrap().parse::<i32>(),
                                        self.gamestate_graph.get_label(&best_successor).unwrap().parse::<i32>()) {
                        if s > b {
                            best_successor = *successor;
                        }
                    } else {
                        panic!("Gamestate label should be an i32!");
                    }
                }
                best_successor.bitxor(gamestate)
            }
            PlayerColor::Red => {
                let mut best_successor: u32 = 0;
                for successor in self.gamestate_graph.out_neighbours(&gamestate) {
                    if best_successor == 0 {
                        best_successor = *successor;
                    }
                    if let (Ok(s), Ok(b)) = (self.gamestate_graph.get_label(successor).unwrap().parse::<i32>(),
                                        self.gamestate_graph.get_label(&best_successor).unwrap().parse::<i32>()) {
                        if s < b {
                            best_successor = *successor;
                        }
                    } else {
                        panic!("Gamestate label should be an i32!");
                    }
                }
                best_successor.bitxor(gamestate)
            }
        }
        

        
    }

    // /// Initializes the gamestate graph with all possible gamestates
    // /// Old version, where graph gets initialized and then evaluated
    // fn initialize_graph(&mut self) -> () {
    //     let initial_gamestate: u32 = 0;

    //     // Initial evaluation of gamestates is -1
    //     self.gamestate_graph.add_vertex_with_label(initial_gamestate, "-1");

    //     let mut unvisited: VecDeque<u32> =  VecDeque::new();
    //     unvisited.push_back(initial_gamestate);

    //     while unvisited.len() != 0 {
    //         let current_gamestate: u32 = unvisited.pop_front().expect("Unvisited Queue should not be empty because of loop invariant");

    //         // Iterate through possible next gamestates, add edge and possible vertex to graph
    //         for next_gamestate in bruteforce_helpers::possible_next_gamestates(current_gamestate) {
    //             // If next gamestate is not in gamestate graph, push to univisted queue
    //             if !self.gamestate_graph.is_vertex_in_graph(&next_gamestate) {
    //                 unvisited.push_back(next_gamestate)
    //             }
    //             self.gamestate_graph.add_vertex_with_label(next_gamestate, "");
    //             self.gamestate_graph.add_edge(current_gamestate, next_gamestate).expect("Gamestates should be in the gamestate graph");
    //         }
    //     }
    // }


    /// Initializes and evaluates the gamestate graph
    /// Uses alpha beta pruning to avoid some gamestates
    /// To do: Fix evaluation
    fn initialize_graph(&mut self) -> () {
        self.gamestate_graph.add_vertex_with_label(0, "0");
        match self.color {
            PlayerColor::Blue => self.alphabeta(0, i32::MIN, i32::MAX, true),
            PlayerColor::Red => self.alphabeta(0, i32::MIN, i32::MAX, false),
        };
    }

    fn alphabeta(&mut self, gamestate: u32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> i32 {
        if is_over(gamestate) {
            match is_won(gamestate) {
                Some(PlayerColor::Blue) => {self.gamestate_graph.set_label(&gamestate, i32::MAX.to_string().as_str())
                                                                .expect("Gamestate should be in graph due to call");
                                            i32::MAX},
                Some(PlayerColor::Red) => {self.gamestate_graph.set_label(&gamestate, i32::MIN.to_string().as_str())
                                                               .expect("Gamestate should be in graph due to call");
                                           i32::MIN},
                None => {self.gamestate_graph.set_label(&gamestate, "0").expect("Gamestate should be in graph due to call"); 0},
            }
        } else {
            match whos_turn_is_it_gamestate(gamestate) {
                PlayerColor::Blue => {
                    // Case where it is blues turn (maximizing player)
                    let mut value: i32 = i32::MIN;
                
                    for next_gamestate in possible_next_gamestates(gamestate) {
                        self.gamestate_graph.add_vertex_with_label(next_gamestate, "0");
                        self.gamestate_graph.add_edge(gamestate, next_gamestate).expect("Gamestate should be in graph due to call");

                        value = value.max(self.alphabeta(next_gamestate, alpha, beta, maximizing_player));
    
                        alpha = alpha.max(value);
                        if maximizing_player {
                            if value > beta {
                                break;
                            }
                        }
                    }
                    if value != i32::MIN {
                        value -= 1;
                    }
                    self.gamestate_graph.set_label(&gamestate, value.to_string().as_str()).expect("Gamestate should be in graph due to call");
                    value
                }
    
                PlayerColor::Red => {
                    // Case where it is reds turn (minimizing player)
                    let mut value: i32 = i32::MAX;
                
                    for next_gamestate in possible_next_gamestates(gamestate) {
                        self.gamestate_graph.add_vertex(next_gamestate);
                        self.gamestate_graph.add_edge(gamestate, next_gamestate).expect("Gamestate should be in graph due to call");

                        value = value.min(self.alphabeta(next_gamestate, alpha, beta, maximizing_player));
    
                        beta = beta.min(value);

                        if !maximizing_player {
                            if value < alpha {
                                break;
                            }
                        }
                    }
                    if value != i32::MAX {
                        value += 1;
                    }

                    self.gamestate_graph.set_label(&gamestate, value.to_string().as_str()).expect("Gamestate should be in graph due to call");
                    value
                }    
            }
        }
    }

}

// to do: Implement tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_graph_test_for_exemplary_edges_in_gamestate_graph() {
        let e = Engine::new(PlayerColor::Blue);
        assert!(!e.gamestate_graph.is_edge_in_graph(0, 1));
        assert!(e.gamestate_graph.is_edge_in_graph(0, 268435456));
        println!("The graph has: {} vertices and: {} edges" , e.gamestate_graph.number_of_vertices(), e.gamestate_graph.number_of_edges());
    }
}