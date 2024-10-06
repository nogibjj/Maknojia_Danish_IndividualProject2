import sqlite3

# Define a global variable for the log file
LOG_FILE = "query_log.md"


def log_query(query):
    """Adds to a query markdown file"""
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")


def general_query(query):
    """Runs a query a user inputs"""
    conn = sqlite3.connect("WRRankingDB.db")
    cursor = conn.cursor()

    cursor.execute(query)

    if (
        query.strip().lower().startswith("insert")
        or query.strip().lower().startswith("update")
        or query.strip().lower().startswith("delete")
    ):
        conn.commit()

    cursor.close()
    conn.close()
    log_query(query)


# Update the `create_record` function to match the WRRankingDB table
def create_record(RK, player_name, team, opp, matchup, start_sit, proj_fpts):
    """Create a record in WRRankingDB"""
    conn = sqlite3.connect("WRRankingDB.db")
    c = conn.cursor()
    c.execute(
        """
        INSERT INTO WRRankingDB 
        (RK, 'PLAYER NAME', TEAM, OPP, MATCHUP, 'START/SIT', 'PROJ. FPTS') 
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
        (RK, player_name, team, opp, matchup, start_sit, proj_fpts),
    )
    conn.commit()
    conn.close()

    log_query(
        f"""INSERT INTO WRRankingDB 
        (RK, 'PLAYER NAME', TEAM, OPP, MATCHUP, 'START/SIT', 'PROJ. FPTS') 
        VALUES ({RK}, {player_name}, {team}, {opp}, {matchup}, {start_sit}, {proj_fpts});
        """
    )


# Update the `update_record` function
def update_record(record_id, RK, player_name, team, opp, matchup, start_sit, proj_fpts):
    """Update a record in WRRankingDB"""
    conn = sqlite3.connect("WRRankingDB.db")
    c = conn.cursor()
    c.execute(
        """
        UPDATE WRRankingDB 
        SET RK=?, 'PLAYER NAME'=?, TEAM=?, OPP=?, MATCHUP=?, 'START/SIT'=?, 'PROJ. FPTS'=? 
        WHERE id=?
        """,
        (RK, player_name, team, opp, matchup, start_sit, proj_fpts, record_id),
    )
    conn.commit()
    conn.close()

    log_query(
        f"""UPDATE WRRankingDB 
        SET RK={RK}, 'PLAYER NAME'={player_name}, TEAM={team}, OPP={opp}, 
        MATCHUP={matchup}, 'START/SIT'={start_sit}, 'PROJ. FPTS'={proj_fpts} 
        WHERE id={record_id};"""
    )


# Update the `delete_record` function
def delete_record(record_id):
    """Delete a record from WRRankingDB"""
    conn = sqlite3.connect("WRRankingDB.db")
    c = conn.cursor()
    c.execute("DELETE FROM WRRankingDB WHERE id=?", (record_id,))
    conn.commit()
    conn.close()

    log_query(f"DELETE FROM WRRankingDB WHERE id={record_id};")


# Update the `read_data` function to read from WRRankingDB
def read_data():
    """Read all data from WRRankingDB"""
    conn = sqlite3.connect("WRRankingDB.db")
    c = conn.cursor()
    c.execute("SELECT * FROM WRRankingDB")
    data = c.fetchall()
    log_query("SELECT * FROM WRRankingDB;")
    return data
