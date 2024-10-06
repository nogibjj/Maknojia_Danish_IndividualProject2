import subprocess


def test_load():
    """Tests the load operation."""
    result = subprocess.run(
        ["python", "main.py", "load"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Transforming and loading data..." in result.stdout


def test_query():
    """Tests the query operation."""
    query_string = "SELECT * FROM WRRankingDB"
    result = subprocess.run(
        ["python", "main.py", "query", query_string],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Querying data" in result.stdout


def test_create_record():
    """Tests the create_record operation."""
    result = subprocess.run(
        [
            "python",
            "main.py",
            "create",
            "1",  # rk
            "Player Name",  # player_name
            "Team A",  # team
            "Team B",  # opp
            "Matchup A",  # matchup
            "start",  # start_sit
            "50.5",  # proj_fpts
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Create Record" in result.stdout


def test_update_record():
    """Tests the update_record operation."""
    result = subprocess.run(
        [
            "python",
            "main.py",
            "update",
            "1",  # record_id
            "2",  # rk
            "Updated Player Name",  # player_name
            "Updated Team A",  # team
            "Updated Team B",  # opp
            "Updated Matchup A",  # matchup
            "bench",  # start_sit
            "60.0",  # proj_fpts
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Update Record" in result.stdout


def test_delete_record():
    """Tests the delete_record operation."""
    result = subprocess.run(
        ["python", "main.py", "delete", "1"],  # record_id
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Delete Record" in result.stdout


if __name__ == "__main__":
    test_load()
    test_query()
    test_create_record()
    test_update_record()
    test_delete_record()
