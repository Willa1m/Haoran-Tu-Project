// Declare each module to make their contents accessible in the project
pub mod data;
pub mod graph;
pub mod analysis;
pub mod utils;

// Re-export selected functionality to external users of the library, if necessary
// This allows users of your library to access these functions/types directly through the library root
pub use data::{load_movies, load_ratings, load_tags};
pub use graph::Graph;
pub use analysis::{compute_shortest_paths, compute_centrality, identify_clusters};
