mod monte_carlo_helpers;

use std::collections::HashMap;

use crate::gamestate_helpers::PlayerColor;
use connect_rust_graphs::graph::Graph;

pub struct Engine {
    color: PlayerColor,
    // Gamestates graph where gamestates have no labels
    gamestate_graph: Graph<u128>,

    // Contains the evaluation of each gamestate
    // The i32 being the evaluation with -1 for each loss, +1 for each win and 0 addd for each draw
    // The u32 is the total number of simulations the gamestate was involved in
    gamestate_evaluations: HashMap<u128, (i32, u32)>,
}

impl Engine {
    /// Returns a montecarlo engine. The gamestate graph only has one vertex with the key 0
    /// (the initial gamestate). The gamestate evaluations only have one pair with 0  and (0,1).
    pub fn new(color: PlayerColor) -> Engine {
        let res: Engine = Engine {
            color,
            gamestate_graph: Graph::new(),
            gamestate_evaluations: HashMap::new(),
        };
        res
    }
}
