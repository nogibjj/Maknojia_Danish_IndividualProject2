use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

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

pub fn load() -> Result<String> {
    let conn = Connection::open("WRRankingDB.db")?;

    conn.execute("DROP TABLE IF EXISTS WRRankingDB", [])?;

    conn.execute(
        "CREATE TABLE WRRankingDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            RK INTEGER,
            'PLAYER NAME' TEXT,
            TEAM TEXT,
            OPP TEXT,
            MATCHUP TEXT,
            'START/SIT' TEXT,
            'PROJ. FPTS' REAL
        )",
        [],
    )?;

    // Here you can add the logic to load data from a CSV or any other source if needed.

    Ok("WRRankingDB.db".to_string())
}

pub fn general_query(query: &str) -> Result<()> {
    let conn = Connection::open("WRRankingDB.db")?;

    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, i32>(1)?,
                row.get::<usize, String>(2)?,
                row.get::<usize, String>(3)?,
                row.get::<usize, String>(4)?,
                row.get::<usize, String>(5)?,
                row.get::<usize, String>(6)?,
                row.get::<usize, f64>(7)?,
            ))
        })?;

        for result in results {
            match result {
                Ok((id, rk, player_name, team, opp, matchup, start_sit, proj_fpts)) => {
                    println!(
                        "Result: id={}, RK={}, player_name={}, team={}, opp={}, matchup={}, start_sit={}, proj_fpts={}",
                        id, rk, player_name, team, opp, matchup, start_sit, proj_fpts
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // Other CUD operations (Insert, Update, Delete)
        conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}

pub fn create_record(
    rk: i32,
    player_name: &str,
    team: &str,
    opp: &str,
    matchup: &str,
    start_sit: &str,
    proj_fpts: f64,
) {
    let conn = Connection::open("WRRankingDB.db").expect("Failed to connect to database");
    conn.execute(
        "INSERT INTO WRRankingDB (RK, 'PLAYER NAME', TEAM, OPP, MATCHUP, 'START/SIT', 'PROJ. FPTS') VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![rk, player_name, team, opp, matchup, start_sit, proj_fpts],
    )
    .expect("Failed to insert record");

    log_query(
        &format!(
            "INSERT INTO WRRankingDB (RK, 'PLAYER NAME', TEAM, OPP, MATCHUP, 'START/SIT', 'PROJ. FPTS') VALUES ({}, '{}', '{}', '{}', '{}', '{}', {});",
            rk, player_name, team, opp, matchup, start_sit, proj_fpts
        ),
        LOG_FILE,
    );
}

pub fn update_record(
    record_id: i32,
    rk: i32,
    player_name: &str,
    team: &str,
    opp: &str,
    matchup: &str,
    start_sit: &str,
    proj_fpts: f64,
) {
    let conn = Connection::open("WRRankingDB.db").expect("Failed to connect to database");
    conn.execute(
        "UPDATE WRRankingDB SET RK = ?1, 'PLAYER NAME' = ?2, TEAM = ?3, OPP = ?4, MATCHUP = ?5, 'START/SIT' = ?6, 'PROJ. FPTS' = ?7 WHERE id = ?8",
        params![rk, player_name, team, opp, matchup, start_sit, proj_fpts, record_id],
    )
    .expect("Failed to update record");

    log_query(
        &format!(
            "UPDATE WRRankingDB SET RK = {}, 'PLAYER NAME' = '{}', TEAM = '{}', OPP = '{}', MATCHUP = '{}', 'START/SIT' = '{}', 'PROJ. FPTS' = {} WHERE id = {};",
            rk, player_name, team, opp, matchup, start_sit, proj_fpts, record_id
        ),
        LOG_FILE,
    );
}

pub fn delete_record(record_id: i32) {
    let conn = Connection::open("WRRankingDB.db").expect("Failed to connect to database");
    conn.execute("DELETE FROM WRRankingDB WHERE id = ?1", params![record_id])
        .expect("Failed to delete record");

    log_query(
        &format!("DELETE FROM WRRankingDB WHERE id = {};", record_id),
        LOG_FILE,
    );
}