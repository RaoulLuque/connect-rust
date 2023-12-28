use connect_rust_graphs::graph::Graph;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use super::Engine;
use crate::helpers::{
    moves::possible_next_gamestates,
    state_of_game::{is_over, is_won},
    PlayerColor,
    PlayerColor::*,
};

// Monte Carlo Selection coefficient
const C: f64 = 0.7;

// If true chooses moves randomly in simulation picker otherwise picks the first move
const CHOOSE_MOVES_RANDOMLY: bool = false;

impl Engine {
    /// Simulates for the given time, anything less than 1000 milliseconds being round up to 1000
    /// and more than 3000 being rounded down to 3000. Proceeds to return the best move according
    /// to which was involved in the most simulations
    /// Time is passed as milliseconds
    pub fn make_move(gamestate: u128, mut time: u128) -> (u128, i8, u32, u128) {
        let timer = Instant::now();

        let color = PlayerColor::Blue;
        let mut gamestate_graph = Graph::new();
        let mut gamestate_evaluations = HashMap::new();

        if time < 1000 {
            time = 1000;
        } else if time > 3000 {
            time = 3000;
        }

        // Check if instant win/end is possible
        let nexts: Vec<u128> = possible_next_gamestates(gamestate)
            .filter(|x| is_over(*x))
            .collect();
        if !nexts.is_empty() {
            return (nexts[0], 0, 0, 0);
        }

        // Reset the gamestate graph in order to avoid paths from down up not leading to gamestate
        // that the game is currently at
        gamestate_graph = Graph::new();
        gamestate_graph.add_vertex(gamestate);
        gamestate_evaluations.entry(gamestate).or_insert((0, 1));

        // Necessary to satisfy compiler, no use in this case since loop is stopped via time
        // running out
        let (_, rx) = std::sync::mpsc::channel();

        // Call the monte carlo loop
        monte_carlo_loop(
            &mut gamestate_graph,
            &mut gamestate_evaluations,
            gamestate,
            color,
            timer,
            time,
            rx,
        );

        // Select which move is best by looking at most
        let move_to_make = gamestate_graph
            .out_neighbours(&gamestate)
            .max_by_key(|x| {
                gamestate_evaluations
                    .get(&x)
                    .expect("Gamestates should be in evaluation hashmap")
                    .1
            })
            .expect("One child should exist");

        println!(
            "Possible next moves after first ganestate: {:?}. The move_to_make is: {:?}",
            gamestate_graph.out_neighbours(&gamestate),
            *move_to_make
        );

        (*move_to_make, 0, 0, 0)
    }
}

/// Loop for monte carlo method and calling the helpersS
pub fn monte_carlo_loop(
    gamestate_graph: &mut Graph<u128>,
    gamestate_evaluations: &mut HashMap<u128, (i32, i32)>,
    gamestate: u128,
    color: PlayerColor,
    timer: Instant,
    time: u128,
    rx: Receiver<bool>,
) {
    // Loop for monte carlo method
    while time as f64 * 0.95 > timer.elapsed().as_millis() as f64 {
        let mut current_node = gamestate;
        let mut last_node = current_node;

        // Selecting until leaf is found
        // Selected node is not in gamestate graph
        while gamestate_graph.is_vertex_in_graph(&current_node) {
            match selection(gamestate_graph, gamestate_evaluations, current_node) {
                None => {
                    break;
                }
                Some(node) => {
                    last_node = current_node;
                    current_node = node;
                }
            }
        }

        // Add new node (current node) to the gamestate graph
        // If game is over just check who won and determine rating accordingly
        let rating: Option<PlayerColor> = match is_over(last_node) {
            false => {
                expand(
                    gamestate_graph,
                    gamestate_evaluations,
                    last_node,
                    current_node,
                );
                simulate_game(current_node)
            }
            true => {
                expand(
                    gamestate_graph,
                    gamestate_evaluations,
                    last_node,
                    current_node,
                );
                is_won(current_node)
            }
        };

        // Propagate result of simulation
        loop {
            backpropagate(gamestate_evaluations, color, current_node, rating);

            if current_node == gamestate {
                break;
            }

            match gamestate_graph.in_neighbours(&current_node).next() {
                Some(a) => current_node = *a,
                None => {
                    println!("Gamestate is: {}. The current node (who should have the parent) is: {}. The last_node is: {}", gamestate, current_node, last_node);
                    panic!("Node doesn't have parent and should have!")
                }
            }
        }
        if let Ok(true) = rx.try_recv() {
            break;
        }
    }
}

/// Selects one of the children of a given node
/// Uses the UCT (Upper confidence bound applied to trees)
/// Returns None if the gamestate is final. Otherwise returns a possible next gamestate
/// according to a formula
fn selection(
    gamestate_graph: &Graph<u128>,
    gamestate_evaluations: &mut HashMap<u128, (i32, i32)>,
    current_gamestate: u128,
) -> Option<u128> {
    // If one of the children is not in gamestate graph, it is selected
    for successor in possible_next_gamestates(current_gamestate) {
        if !gamestate_graph.is_vertex_in_graph(&successor) {
            return Some(successor);
        }
    }

    // If gamestate is final return gamestate
    if is_over(current_gamestate) {
        return None;
    }

    // Check whether gamestate has been visited less than 30 times
    if gamestate_evaluations
        .get(&current_gamestate)
        .expect("Gamestate should be in gamestate evaluations")
        .1
        < 30
    {
        return Some(simulation_picker(current_gamestate));
    } else {
        // By case all of the children are in the gamestate graph
        let mut best_successor = 0;
        for successor in possible_next_gamestates(current_gamestate) {
            if best_successor == 0 {
                best_successor = successor;
            } else {
                // Compare evaluation formula
                if (gamestate_evaluations
                    .get(&successor)
                    .expect("Successor should be in gamestate evaluations")
                    .0 as f64
                    / gamestate_evaluations
                        .get(&successor)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64
                    + C * ((gamestate_evaluations
                        .get(&current_gamestate)
                        .expect("Successor should be in gamestate evaluations")
                        .1 as f64)
                        .ln()
                        / (gamestate_evaluations
                            .get(&successor)
                            .expect("Successor should be in gamestate evaluations")
                            .1 as f64)))
                    >= (gamestate_evaluations
                        .get(&best_successor)
                        .expect("Successor should be in gamestate evaluations")
                        .0 as f64
                        / gamestate_evaluations
                            .get(&best_successor)
                            .expect("Successor should be in gamestate evaluations")
                            .1 as f64
                        + C * ((gamestate_evaluations
                            .get(&current_gamestate)
                            .expect("Successor should be in gamestate evaluations")
                            .1 as f64)
                            .ln()
                            / (gamestate_evaluations
                                .get(&best_successor)
                                .expect("Successor should be in gamestate evaluations")
                                .1 as f64)))
                {
                    best_successor = successor;
                }
            }
        }
        if best_successor != 0 {
            return Some(best_successor);
        } else {
            return None;
        }
    }
}

/// Adds the current node to the gamestate graph, an edge between last and current node
/// and current node to gamestate evaluations with initial value (0,1)
fn expand(
    gamestate_graph: &mut Graph<u128>,
    gamestate_evaluations: &mut HashMap<u128, (i32, i32)>,
    last_node: u128,
    current_node: u128,
) {
    gamestate_graph.add_vertex(current_node);
    gamestate_graph
        .add_edge(last_node, current_node)
        .expect("Gamestates should be in graph");

    gamestate_evaluations.entry(current_node).or_insert((0, 1));
}

/// Simulates a game starting from starting_node
/// Returns an Option with a player color with the player that won the simulated game
/// or none in case it was a draw
fn simulate_game(starting_node: u128) -> Option<PlayerColor> {
    let mut current_gamestate = starting_node;
    while !is_over(current_gamestate) {
        current_gamestate = simulation_picker(current_gamestate)
    }
    is_won(current_gamestate)
}

/// Returns what the next gamestate of the simulation should be
fn simulation_picker(current_gamestate: u128) -> u128 {
    let vec: Vec<u128> = possible_next_gamestates(current_gamestate).collect();
    if CHOOSE_MOVES_RANDOMLY {
        *vec.choose(&mut rand::thread_rng())
            .expect("Gamestate shouldn't be final and as such should have children")
    } else {
        *vec.get(0)
            .expect("Gamestate shouldn't be final and as such should have children")
    }
}

/// Propagates the rating of the simulated game to the parent nodes.
/// -1 is added to the rating of each node if the simulated game was lost,
/// 1 if won and 0 if it was a draw
fn backpropagate(
    gamestate_evaluations: &mut HashMap<u128, (i32, i32)>,
    color: PlayerColor,
    node: u128,
    rating: Option<PlayerColor>,
) {
    let rating = match rating {
        None => 0,
        Some(Blue) => match color {
            Blue => 1,
            Red => -1,
        },
        Some(Red) => match color {
            Blue => -1,
            Red => 1,
        },
    };

    let eval = gamestate_evaluations.entry(node).or_insert((0, 1));
    eval.0 += rating;
    eval.1 += 1;
}
