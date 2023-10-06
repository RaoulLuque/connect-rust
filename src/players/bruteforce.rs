mod bruteforce_helpers;

use std::ops::BitXor;

use crate::gamestate_helpers::{PlayerColor, whos_turn_is_it_gamestate, is_over, is_won};
use connect_rust_graphs::graph::Graph;
use bruteforce_helpers::possible_next_gamestates;

pub struct Engine {
    color: PlayerColor, 
    // Gamestates graph where gamestates have labels with their evaluation as i32.
    // "" is the initial evaluation
    // Evaluation +i32::max-x stands for the blue player winning in x turns 
    // Evaluation -i32::max+x stands for the red player winning in x turns
    gamestate_graph: Graph<u128>,

    visits: u32,
}


impl Engine {
    /// Constructor for new engine constructing empty gamestate graph
    pub fn new(color: PlayerColor) -> Engine {
        let mut res: Engine = Engine{color, gamestate_graph: Graph::new(), visits: 0};
        
        res.initialize_graph();

        res


    }

    /// Returns the best possible move accounting to the gamestate graph calculated at initialization
    pub fn make_move(&self, gamestate: u128) -> u128 {
        match self.color {
            PlayerColor::Blue => {
                // println!("Rating of position blue can win: {}", self.gamestate_graph.get_label(&2779152705).unwrap());
                // println!("Rating of position blue wins: {}", self.gamestate_graph.get_label(&2779156801).unwrap());

                let mut best_successor: u128 = 0;
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
                let mut best_successor: u128 = 0;
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


    /// Initializes and evaluates the gamestate graph
    /// Uses alpha beta pruning to avoid some gamestates
    fn initialize_graph(&mut self) -> () {
        self.gamestate_graph.add_vertex_with_label(0, "0");
        match self.color {
            PlayerColor::Blue => self.alphabeta(0, i32::MIN, i32::MAX, true),
            PlayerColor::Red => self.alphabeta(0, i32::MIN, i32::MAX, false),
        };
    }

    fn alphabeta(&mut self, gamestate: u128, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> i32 {
        if self.visits % 1000000 == 0 {
            println!("The number of visited nodes is: {}", self.visits);
        }

        self.visits += 1;
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