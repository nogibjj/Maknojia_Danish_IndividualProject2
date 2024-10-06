# Danish_Maknojia_MP5

[![CI](https://github.com/nogibjj/Maknojia_Danish_MP5/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Maknojia_Danish_MP5/actions/workflows/cicd.yml)

## Project Overview
This project is a python script which interacts with a dataset on SQLlite. The project focuses on the ETL pipeline (Extract, Transform & Load):

- Extract: Raw data is accessed via github repo

- Transform & Load: Reads data from a CSV file, processes it, and loads it into a local SQLite3 database named WRRankingDB.db.

- Query: Performs CRUD operations (create, read, update, delete) on the database, and logging each SQL query to a markdown file (query_log.md). The log_query function appends the executed SQL queries to the log file, while general_query, create_record, update_record, delete_record, and read_data manage data operations within the database.

## Directory Overview

```
├── Makefile
├── README.md
├── WRRankingDB.db
├── __pycache__
│   ├── main.cpython-312.pyc
│   └── test.cpython-312-pytest-7.4.0.pyc
├── data
│   └── WRRankingsWeek5.csv
├── lib
│   ├── __init__.py
│   ├── __pycache__
│   │   ├── __init__.cpython-312.pyc
│   │   ├── query.cpython-312.pyc
│   │   └── transform_load.cpython-312.pyc
│   ├── extract.py
│   ├── query.py
│   └── transform_load.py
├── main.py
├── query_log.md
├── requirement.txt
└── test.py

```

## CI/CD
1. make all: Will test, format, and lint the project

