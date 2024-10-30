use danish_maknojia::{extract, query, transform_load}; // Updated to match your lib.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            extract(
                "https://github.com/danishmaknojia/data/raw/refs/heads/main/WRRankingsWeek5Ranking.csv",
                "data/WRRankingsWeek5.csv",
                "data",
            );
        }
        "transform_load" => match transform_load("data/WRRankingsWeek5.csv") {
            Ok(_) => println!("Data loaded successfully!"),
            Err(err) => eprintln!("Error: {:?}", err),
        },
        "query" => {
            if let Some(q) = args.get(2) {
                if let Err(err) = query(q) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}
