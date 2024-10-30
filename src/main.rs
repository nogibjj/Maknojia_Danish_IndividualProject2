use danish_maknojia::{
    query::{create_record, delete_record, general_query, update_record},
    transform_load::load,
};
use std::env;

fn main() {
    // Step 1: Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "load" => {
            // Load data
            println!("Loading data into WRRankingDB...");
            if let Err(err) = load() {
                eprintln!("Error during load: {:?}", err);
            } else {
                println!("Data loaded successfully!");
            }
        }
        "query" => {
            // Run a general SQL query
            if let Some(query) = args.get(2) {
                if let Err(err) = general_query(query) {
                    eprintln!("Error: {:?}", err);
                } else {
                    println!("Query executed successfully!");
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        "create" => {
            // Create a new record
            if args.len() == 9 {
                let rk = args[2].parse::<i32>().expect("Invalid rk value");
                let player_name = &args[3];
                let team = &args[4];
                let opp = &args[5];
                let matchup = &args[6];
                let start_sit = &args[7];
                let proj_fpts = args[8].parse::<f64>().expect("Invalid proj_fpts value");

                println!("Creating record...");
                create_record(rk, player_name, team, opp, matchup, start_sit, proj_fpts);
                println!("Record created successfully!");
            } else {
                println!(
                    "Usage: {} create <rk> <player_name> <team> <opp> <matchup> <start_sit> <proj_fpts>",
                    args[0]
                );
            }
        }
        "update" => {
            // Update an existing record
            if args.len() == 10 {
                let record_id = args[2].parse::<i32>().expect("Invalid record_id value");
                let rk = args[3].parse::<i32>().expect("Invalid rk value");
                let player_name = &args[4];
                let team = &args[5];
                let opp = &args[6];
                let matchup = &args[7];
                let start_sit = &args[8];
                let proj_fpts = args[9].parse::<f64>().expect("Invalid proj_fpts value");

                println!("Updating record...");
                update_record(
                    record_id,
                    rk,
                    player_name,
                    team,
                    opp,
                    matchup,
                    start_sit,
                    proj_fpts,
                );
                println!("Record updated successfully!");
            } else {
                println!(
                    "Usage: {} update <record_id> <rk> <player_name> <team> <opp> <matchup> <start_sit> <proj_fpts>",
                    args[0]
                );
            }
        }
        "delete" => {
            // Delete a record
            if args.len() == 3 {
                let record_id = args[2].parse::<i32>().expect("Invalid record_id value");

                println!("Deleting record...");
                delete_record(record_id);
                println!("Record deleted successfully!");
            } else {
                println!("Usage: {} delete <record_id>", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'load', 'query', 'create', 'update', or 'delete'.");
        }
    }
}
