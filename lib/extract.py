"""
Extract a dataset from a URL the Week 5 Fantasy Football PPR WR 
"""

import os
import requests


def extract(
    url="""
    https://raw.githubusercontent.com/nogibjj/Maknojia_Danish_MP5/refs/heads/main/data/WRRankingsWeek5.csv?token=GHSAT0AAAAAACWUAB4MNDYOLNSZFJMUMKH2ZYBZBPQ 
    """,
    file_path="data/WRRankingsWeek5.csv",
    directory="data",
):
    """Extract a url to a file path"""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, "wb") as f:
            f.write(r.content)
    return file_path
