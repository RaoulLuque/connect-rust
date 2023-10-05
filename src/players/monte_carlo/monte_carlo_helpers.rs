use std::time::Instant;
use std::ops::BitXor;
use connect_rust_graphs::graph::Graph;
use rand::seq::SliceRandom;

use crate::gamestate_helpers::{possible_next_gamestates, is_over, PlayerColor, PlayerColor::*, is_won};
use super::Engine;

// Monte Carlo Selection coefficient
const C: f64 = 0.7;


impl Engine {
    // Time is passed as milliseconds
    pub fn make_move(&mut self, gamestate: u32, mut time: u128) -> u32 {
        let timer = Instant::now();

        if time < 1000 {
            time = 1000;
        } else if time > 3000 {
            time = 3000;
        }

        time = 5000;

        // Check if instant win/end is possible
        let nexts: Vec<u32> = possible_next_gamestates(gamestate).filter(|x| is_over(*x)).collect();
        if !nexts.is_empty() {
            return (nexts[0]).bitxor(gamestate);
        }
        

        // Reset the gamestate graph in order to avoid paths down up not leading to gamestate
        // that the game is currently at
        self.gamestate_graph = Graph::new();
        self.gamestate_graph.add_vertex(gamestate);
        self.gamestate_evaluations.entry(gamestate).or_insert((0,1));

        while time as f64 * 0.95 > timer.elapsed().as_millis() as f64 {
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

            // Add new node (current node) to the gamestate graph
            self.expand(last_node, current_node);
            let rating: Option<PlayerColor> = Engine::simulate_game(current_node);

            // println!("Beginning to propagate: {}", current_node);
            loop {
                // println!("Propagating {}", current_node);
                self.backpropagate(current_node, rating);

                if current_node == gamestate {
                    break;
                }
                match self .gamestate_graph
                                    .in_neighbours(&current_node)
                                    .next() {
                    Some(a) => {current_node = *a},
                    None => {println!("Gamestate is: {}. The current node is: {}. The last_node is: {}", gamestate, current_node, last_node);
                            panic!("Node doesn't have parent and should have!")},
                }
            }
        }

        println!("Number of vertices in gamestate graph: {}", self.gamestate_graph.number_of_vertices());
        println!("Number of simulations involving current gamestate: {}", self.gamestate_evaluations.get(&gamestate).unwrap().1);

        self.gamestate_graph
            .out_neighbours(&gamestate)
            .max_by_key(|x| self .gamestate_evaluations
                                        .get(&x)
                                        .expect("Gamestates should be in evaluation hashmap")
                                        .1)
            .expect("One child should exist")
            .bitxor(gamestate)
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

        // If gamestate is final return gamestate
        if is_over(current_gamestate) {
            return None;
        }

        // Check whether gamestate has been visited less than 30 times
        if self .gamestate_evaluations
                .get(&current_gamestate)
                .expect("Gamestate should be in gamestate evaluations")
                .1 < 30 {
            return Some(Engine::simulation_picker(current_gamestate));
    
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

    /// Adds the current node to the gamestate graph, an edge between last and current node
    /// and current node to gamestate evaluations with inital value (0,1)
    fn expand(&mut self, last_node: u32, current_node: u32) {
        self.gamestate_graph.add_vertex(current_node);
        self.gamestate_graph.add_edge(last_node, current_node).expect("Gamestates should be in graph");

        self.gamestate_evaluations.entry(current_node).or_insert((0,1));
    }

    fn simulate_game(starting_node: u32) -> Option<PlayerColor> {
        let mut current_gamestate = starting_node;
        while !is_over(current_gamestate) {
            current_gamestate = Engine::simulation_picker(current_gamestate)
        }
        is_won(current_gamestate)
    }

    fn simulation_picker (current_gamestate: u32) -> u32 {
        let vec: Vec<u32> = possible_next_gamestates(current_gamestate).collect();
        *vec.choose(&mut rand::thread_rng()).expect("Gamestate shouldn't be final")
    }

    /// Propagate the rating of the simulated game to the parent nodes.
    /// -1 is added to the rating of each node if the simulated game was lost,
    /// 1 if won and 0 if it was a draw
    fn backpropagate(&mut self, node: u32, rating: Option<PlayerColor>) {
        let rating = match rating {
            None => 0,
            Some(Blue) => match self.color {Blue => 1, Red => -1},
            Some(Red) => match self.color {Blue => -1, Red => 1},
        };

        let eval = self.gamestate_evaluations.entry(node).or_insert((0,1));
        eval.0 += rating;
        eval.1 += 1;
    }
}