mod data_preparation;
mod graph_construction;
mod six_degrees;
mod degree_distribution;
mod densest_subgraph;
mod  centrality_measures;
mod algorithms;

fn main() {
    println!("Starting the Video Game Platform Graph Analysis...");

    let data_path = "video_game_graph_analysis\\data\\statistic_id1391589_number-of-major-upcoming-video-games-2022-2024-by-platform.xlsx";

     // Step 1: Data Preparation
     println!("Reading game data...");
     let game_releases = data_preparation::read_game_data(&data_path)
         .expect("Failed to read game data. Please check the file path and format.");
 
     // Step 2: Graph Construction
     println!("Constructing the graph...");
     let mut game_graph = graph_construction::GameGraph::new();

     //pass a slice of the vector
     game_graph.build_graph(&game_releases);

     // Optionally, display some information about the graph
     println!("Graph has been constructed with {} nodes and {} edges.",
     game_graph.graph.node_count(), game_graph.graph.edge_count());
 
     // Display some information about the graph (optional)
     println!("Graph construction complete.");
     println!("Number of nodes in the graph: {}", game_graph.graph.node_count());
     println!("Number of edges in the graph: {}", game_graph.graph.edge_count());

    // Step 3: Apply Graph Algorithms
    // Example: Compute Degree Distribution
    println!("Analyzing degree distribution...");
    let degrees = degree_distribution::compute_degrees(&game_graph);
    degree_distribution::analyze_degree_distribution(&degrees);

    // Compute Centrality Measures
    println!("Calculating centrality measures...");
    let centralities = centrality_measures::closeness_centrality(&game_graph);
    for (platform, centrality) in centralities {
        println!("Closeness Centrality of {}: {}", platform, centrality);
    }

    // Compute Six Degrees of Separation
    println!("Calculating average path length...");
    let avg_path_length = six_degrees::average_path_length(&game_graph);
    println!("Average path length in the graph: {}", avg_path_length);

    // Find the Densest Subgraph
    println!("Finding the densest subgraph...");
    let (densest_subgraph, density) = densest_subgraph::find_densest_subgraph(&game_graph);
    println!("Densest Subgraph has density {}: {:?}", density, densest_subgraph);

    println!("Analysis complete.");
}
