use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use crate::data_preparation::GameRelease; 


pub struct GameGraph {
    pub graph: DiGraph<String, u32>,           // Graph storing platforms with edge weights
    pub index_map: HashMap<String, NodeIndex>, // Mapping from platform names to node indices
}

impl GameGraph {
    /// Creates a new empty GameGraph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    /// Adds a platform to the graph if it doesn't already exist.
    pub fn add_platform(&mut self, platform: &str) -> NodeIndex {
        *self.index_map.entry(platform.to_string()).or_insert_with(|| {
            self.graph.add_node(platform.to_string())
        })
    }

    /// Builds the graph based on a vector of game releases.
    pub fn build_graph(&mut self, game_releases: &[GameRelease]) {
        for release in game_releases {
            let node_index = self.add_platform(&release.platform);

            // Example: Connect this node to other nodes based on some condition
            for other_release in game_releases {
                if other_release.platform != release.platform { // Ensure no self-loops and connections are meaningful
                    let other_node_index = self.add_platform(&other_release.platform);
                    // Optionally check other conditions to connect nodes, e.g., same release year
                    if other_release.year == release.year {
                        self.graph.add_edge(node_index, other_node_index, 1); // Weight can be adjusted or made dynamic
                    }
                }
            }
        }
    }

    pub fn add_release_edge(&mut self, from_platform: &str, to_platform: &str) {
        let from_index = self.add_platform(from_platform);
        let to_index = self.add_platform(to_platform);
        // Check if the edge already exists; if not, add it with initial weight
        if self.graph.find_edge(from_index, to_index).is_none() {
            self.graph.add_edge(from_index, to_index, 1);
        }
    }

}


