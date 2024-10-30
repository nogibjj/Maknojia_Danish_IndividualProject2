use danish_maknojia::{transform_load::load, query::{general_query, create_record}};

#[test]
fn test_load() {
    let dataset = "data/WRRanking.csv";
    let result = load(dataset);

    assert_eq!(result.unwrap(), "WRRankingDB.db");
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

    create_record(rk, player_name, team, opp, matchup, start_sit, proj_fpts);

    // Optionally, verify insertion by querying the database
    let select_query = "SELECT * FROM WRRankingDB WHERE RK = 1;";
    let result = general_query(select_query);
    
    assert!(result.is_ok());
}

#[test]
fn test_general_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM WRRankingDB WHERE MATCHUP = 'Matchup X';";
    let result = general_query(select_query);

    assert!(result.is_ok());
}
