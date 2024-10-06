"""
Transforms and Loads data into the local SQLite3 database
"""

import sqlite3
import csv


# load the csv file and insert into a new sqlite3 database
def load(dataset="data/WRRankingsWeek5.csv"):
    """Transforms and Loads data into the local SQLite3 database"""
    payload = csv.reader(open(dataset, newline=""), delimiter=",")
    # skips the header of csv
    next(payload)
    conn = sqlite3.connect("WRRankingDB.db")
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS WRRankingDB")
    c.execute(
        """
        CREATE TABLE WRRankingDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            RK INTEGER,
            'PLAYER NAME' TEXT,
            TEAM TEXT,
            OPP TEXT,
            MATCHUP TEXT,
            'START/SIT' TEXT,
            'PROJ. FPTS' FLOAT
        )
    """
    )
    # insert
    c.executemany(
        """
        INSERT INTO WRRankingDB (
        RK,'PLAYER NAME',TEAM,OPP,MATCHUP,'START/SIT','PROJ. FPTS'
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?)""",
        payload,
    )
    conn.commit()
    conn.close()
    return "WRRankingDB.db"
