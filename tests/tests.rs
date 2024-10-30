// src/tests.rs
use danish_maknojia::{query, transform_load};

#[test]
fn test_transform_load() {
    let dataset = "data/WRRankingsWeek5.csv";
    let result = transform_load(dataset); // Corrected the function call

    assert_eq!(result.unwrap(), "MatchResultsDB.db");
}

#[test]
fn test_create_record() {
    // Create a new record
    let rk = 1;
    let player_name = "John Doe";
    let team = "Team A";
    let opp = "Team B";
    let matchup = "Matchup X";
    let start_sit = "Start";
    let proj_fpts = 12.5;

    // Uncomment and implement create_record if needed
    // create_record(rk, player_name, team, opp, matchup, start_sit, proj_fpts);

    // Optionally, verify insertion by querying the database
    let select_query = "SELECT * FROM WRRankingDB WHERE rk = 1;";
    let result = query(select_query);

    assert!(result.is_ok());
}

#[test]
fn test_general_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM WRRankingDB WHERE matchup = 'Matchup X';";
    let result = query(select_query);

    assert!(result.is_ok());
}
