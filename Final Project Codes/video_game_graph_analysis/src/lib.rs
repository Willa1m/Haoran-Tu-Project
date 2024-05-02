// Declare modules to organize the codebase
// src/lib.rs
pub mod data_preparation;
pub mod graph_construction;
pub mod six_degrees;
pub mod degree_distribution;
pub mod densest_subgraph;
pub mod algorithms;
pub mod tests;
pub mod centrality_measures;

// Re-exporting the main components to be accessible from the library
pub use data_preparation::read_game_data;
pub use graph_construction::GameGraph;
pub use six_degrees::average_path_length;
pub use degree_distribution::{compute_degrees, analyze_degree_distribution};
pub use densest_subgraph::find_densest_subgraph;
pub use algorithms::{breadth_first_search, depth_first_search};
pub use centrality_measures::closeness_centrality;

//define some common structs or enums that are used across several modules
#[derive(Debug, Clone)]
pub struct GameRelease {
    pub platform: String,
    pub year: u32,
}

// define them here
pub fn common_utility_function() -> String {
    // Just an example function
    "Hello, this is a utility function!".to_string()
}
