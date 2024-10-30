// src/lib.rs
use csv; // Ensure the csv crate is imported
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write; // Import the Error trait for custom error handling

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");
    println!("Extraction successful!");
}

pub fn transform_load(dataset: &str) -> Result<String, Box<dyn Error>> {
    // Updated to return a Box<dyn Error>
    let conn = Connection::open("MatchResultsDB.db")?;

    conn.execute("DROP TABLE IF EXISTS WRRankingDB", [])?;

    conn.execute(
        "CREATE TABLE WRRankingDB (
            rk INTEGER,
            player_name TEXT,
            team TEXT,
            opp TEXT,
            matchup TEXT,
            start_sit TEXT,
            proj_fpts REAL
        )",
        [],
    )?;

    // Use the csv crate to read the dataset
    let mut rdr = csv::Reader::from_path(dataset).map_err(|e| format!("CSV Error: {}", e))?; // Convert CSV errors to String

    let mut stmt = conn.prepare(
        "INSERT INTO WRRankingDB (
            rk, 
            player_name, 
            team, 
            opp, 
            matchup, 
            start_sit, 
            proj_fpts
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        let record = result.map_err(|e| format!("CSV Record Error: {}", e))?; // Convert CSV record errors to String
        stmt.execute(&[
            &record[0], // rk
            &record[1], // player_name
            &record[2], // team
            &record[3], // opp
            &record[4], // matchup
            &record[5], // start_sit
            &record[6], // proj_fpts
        ])?;
    }

    Ok("MatchResultsDB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("MatchResultsDB.db")?;
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,    // rk
                row.get::<usize, String>(1)?, // player_name
                row.get::<usize, String>(2)?, // team
                row.get::<usize, String>(3)?, // opp
                row.get::<usize, String>(4)?, // matchup
                row.get::<usize, String>(5)?, // start_sit
                row.get::<usize, f64>(6)?,    // proj_fpts
            ))
        })?;

        for result in results {
            match result {
                Ok((rk, player_name, team, opp, matchup, start_sit, proj_fpts)) => {
                    println!(
                        "Result: rk={}, player_name={}, team={}, opp={}, matchup={}, start_sit={}, proj_fpts={}",
                        rk, player_name, team, opp, matchup, start_sit, proj_fpts
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
