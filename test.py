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
            # Rank
            "1",
            "Player Name",
            # Team
            "Team A",
            # Opponent
            "Team B",
            # Matchup
            "Matchup A",
            # Start_sit
            "start",
            # proj_fpts
            "50.5",
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
            "1",
            "2",
            "Updated Player Name",
            "Updated Team A",
            "Updated Team B",
            "Updated Matchup A",
            "bench",
            "60.0",
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
        ["python", "main.py", "delete", "1"],
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
