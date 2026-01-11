use arcivis_layout::{ArciVisLayout, Edge, Node, generate_svg};
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct Input {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.json> <output.svg>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // Read and parse JSON
    let json_content = fs::read_to_string(input_file)?;
    let input: Input = serde_json::from_str(&json_content)?;

    // Convert edges to tuple format
    let edges_tuples: Vec<(usize, usize, Option<usize>, Option<usize>)> = input
        .edges
        .iter()
        .map(|e| (e.source, e.target, e.source_port, e.target_port))
        .collect();

    // Create layout config
    let layout = ArciVisLayout::default();

    // Run layout
    let result = layout.layout(input.nodes, edges_tuples);

    // Generate SVG
    generate_svg(&result, output_file, false, false);

    println!("SVG generated: {}", output_file);

    Ok(())
}