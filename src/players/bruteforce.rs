mod bruteforce_helpers;

use std::ops::BitXor;

use crate::gamestate_helpers::{PlayerColor, whos_turn_is_it_gamestate, is_over, is_won};
use connect_rust_graphs::graph::Graph;
use bruteforce_helpers::{possible_next_gamestates, mirror_gamestate};

pub struct Engine {
    color: PlayerColor,
    // Gamestates graph where gamestates have labels with their evaluation as i32.
    // "" is the initial evaluation
    // Evaluation +i32::max-x stands for the blue player winning in x turns
    // Evaluation -i32::max+x stands for the red player winning in x turns
    // Gamestate that are equal up to mirroring are considered the same and only
    // the gamestate with higher encoding value according to the above encoding method is saved
    // as representative
    gamestate_graph: Graph<u128>,

    // Visits for tracking progress
    visits: u128,

}


impl Engine {
    /// Constructor for new engine constructing empty gamestate graph
    pub fn new(color: PlayerColor) -> Engine {
        let mut res: Engine = Engine{color, gamestate_graph: Graph::new(), visits: 0};

        res.initialize_graph();

        println!("Number of nodes: {}", res.gamestate_graph.number_of_vertices());

        res


    }

    /// Returns the best possible move according to the gamestate graph calculated at initialization
    /// Considers mirroring used in gamestate graph to reduce number of considered gamestates
    pub fn make_move(&self, gamestate: u128) -> u128 {
        // Mirror gamestate if not the one used in gamestate graph, i.e. not the one with
        // higher encoding value
        let mirrored: bool = gamestate < mirror_gamestate(gamestate);

        let mut gamestate_mirrored = gamestate;
        if mirrored {
            gamestate_mirrored = mirror_gamestate(gamestate_mirrored);
        }

        let mut best_successor = match self.color {
            PlayerColor::Blue => {


                let mut best_successor: u128 = 0;
                for successor in self.gamestate_graph.out_neighbours(&gamestate_mirrored) {
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
                best_successor
            }

            PlayerColor::Red => {
                let mut best_successor: u128 = 0;

                for successor in self.gamestate_graph.out_neighbours(&gamestate_mirrored) {
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
                best_successor
            }
        };

        //Check if the best successor is mirrored from mirror gamestates perspective (i.e. not mirrored)
        if !best_successor.bitxor(gamestate_mirrored).is_power_of_two() {
            best_successor = mirror_gamestate(best_successor);
        }
        best_successor = best_successor.bitxor(gamestate_mirrored);

        if mirrored {
            mirror_gamestate(best_successor)
        } else {
            best_successor
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
        if self.gamestate_graph.number_of_vertices() % 1000000 == 0 {
            println!("Number of vertices in gamestate Graph is: {}", self.gamestate_graph.number_of_vertices());
        }
        self.visits += 1;
        if self.visits % 1000000 == 0 {
            println!("Number of visits to alphabeta function is: {}", self.visits);
        }

        if is_over(gamestate) {
            match is_won(gamestate) {
                Some(PlayerColor::Blue) =>  {self.gamestate_graph.set_label(&gamestate, i32::MAX.to_string().as_str())
                    .expect("Gamestate should be in graph due to call");
                    i32::MAX},
                Some(PlayerColor::Red) =>   {self.gamestate_graph.set_label(&gamestate, i32::MIN.to_string().as_str())
                    .expect("Gamestate should be in graph due to call");
                    i32::MIN},
                None =>     {self.gamestate_graph   .set_label(&gamestate, "0")
                    .expect("Gamestate should be in graph due to call");
                    0},
            }
        } else {
            match whos_turn_is_it_gamestate(gamestate) {
                PlayerColor::Blue => {
                    // Case where it is blues turn (maximizing player)
                    let mut value: i32 = i32::MIN;

                    for next_gamestate in possible_next_gamestates(gamestate) {
                        // Check for mirroring and use gamestate that has higher encoding value as representative
                        let mut next_gamestate_mirrored = next_gamestate;
                        if mirror_gamestate(next_gamestate_mirrored) >= next_gamestate_mirrored {
                            next_gamestate_mirrored = mirror_gamestate(next_gamestate);
                        }
                        if self.gamestate_graph.is_vertex_in_graph(&next_gamestate_mirrored){
                            value = value.max(self.gamestate_graph
                                .get_label(&next_gamestate_mirrored)
                                .expect("Gamestate should have label")
                                .parse::<i32>()
                                .expect("label should be an i32"));
                            self.gamestate_graph.add_edge(gamestate, next_gamestate_mirrored).expect("Gamestate should be in graph due to call");
                        } else {
                            self.gamestate_graph.add_vertex_with_label(next_gamestate_mirrored, "0");
                            self.gamestate_graph.add_edge(gamestate, next_gamestate_mirrored).expect("Gamestate should be in graph due to call");
                            value = value.max(self.alphabeta(next_gamestate_mirrored, alpha, beta, maximizing_player));
                        }
                        alpha = alpha.max(value);
                        if maximizing_player {
                            if value > beta {
                                break;
                            }
                        }
                    }
                    if value != 0 && value != i32::MIN {
                        value -= 1;
                    }
                    self.gamestate_graph.set_label(&gamestate, value.to_string().as_str()).expect("Gamestate should be in graph due to call");
                    value
                }

                PlayerColor::Red => {
                    // Case where it is reds turn (minimizing player)
                    let mut value: i32 = i32::MAX;

                    for next_gamestate in possible_next_gamestates(gamestate) {
                        // Check for mirroring and use gamestate that has higher encoding value as representative
                        let mut next_gamestate_mirrored = next_gamestate;
                        if mirror_gamestate(next_gamestate_mirrored) >= next_gamestate_mirrored {
                            next_gamestate_mirrored = mirror_gamestate(next_gamestate_mirrored);
                        }

                        if self.gamestate_graph.is_vertex_in_graph(&next_gamestate_mirrored) {
                            value = value.min(self.gamestate_graph
                                .get_label(&next_gamestate_mirrored)
                                .expect("Gamestate should have label")
                                .parse::<i32>()
                                .expect("label should be an i32"));
                            self.gamestate_graph.add_edge(gamestate, next_gamestate_mirrored).expect("Gamestate should be in graph due to call");
                        } else {
                            self.gamestate_graph.add_vertex_with_label(next_gamestate_mirrored, "0");
                            self.gamestate_graph.add_edge(gamestate, next_gamestate_mirrored).expect("Gamestate should be in graph due to call");
                            value = value.min(self.alphabeta(next_gamestate_mirrored, alpha, beta, maximizing_player));
                        }

                        beta = beta.min(value);

                        if !maximizing_player {
                            if value < alpha {
                                break;
                            }
                        }
                    }
                    if value != 0 && value != i32::MAX {
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