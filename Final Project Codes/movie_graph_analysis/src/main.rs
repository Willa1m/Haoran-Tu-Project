// Import the necessary modules
mod data;
mod graph;
mod analysis;
mod utils;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn write_to_file(path: &Path, data: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    // Load movies, ratings, and tags from CSV files
    let movies = match data::load_movies("data/movies.csv") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to load movies: {}", e);
            return;
        }
    };

    let ratings = match data::load_ratings("data/ratings.csv") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to load ratings: {}", e);
            return;
        }
    };

    let tags = match data::load_tags("data/tags.csv") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load tags: {}", e);
            return;
        }
    };

    // Create the graph from loaded data
    let movie_graph = graph::create_graph(&movies, &ratings, &tags);

    // Run analyses on the created graph
    analysis::run_analyses(&movie_graph);

    let movie_graph = graph::create_graph(&movies, &ratings, &tags);

    // Call to debug graph information
    graph::debug_graph_info(&movie_graph);

    // Continue with any further analyses
    analysis::run_analyses(&movie_graph);
    
    let output_path = Path::new("data/output.txt");
    let output_data = format!("Total nodes: {}\nTotal edges: {}\n", movie_graph.num_nodes(), movie_graph.num_edges());

    // Writing analysis results to a file
    if let Err(e) = write_to_file(output_path, &output_data) {
        eprintln!("Failed to write to file: {}", e);
    }

}
