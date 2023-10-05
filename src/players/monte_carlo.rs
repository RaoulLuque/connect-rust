mod monte_carlo_helpers;

use std::collections::HashMap;

use crate::gamestate_helpers::PlayerColor;
use connect_rust_graphs::graph::Graph;


pub struct Engine {
    color: PlayerColor, 
    // Gamestates graph where gamestates have no labels
    gamestate_graph: Graph<u32>,

    // Contains the evaluation of each gamestate
    // The i32 being the evaluation with -1 for each loss, +1 for each win and 0 addd for each draw
    // The u32 is the total number of simulations the gamestate was involved in
    gamestate_evaluations: HashMap<u32, (i32,u32)>,
}

impl Engine {
    pub fn new(color: PlayerColor) -> Engine {
        let mut res: Engine = Engine {color, gamestate_graph: Graph::new(), gamestate_evaluations: HashMap::new()};
        res.gamestate_graph.add_vertex(0);
        res.gamestate_evaluations.insert(0, (0,1));

        res
    }

    
}

