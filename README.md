# Danish Maknojia IDS706 Individual Project 2

[![Rust CI/CD Pipeline](https://github.com/nogibjj/Maknojia_Danish_IndividualProject2/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Maknojia_Danish_IndividualProject2/actions/workflows/cicd.yml)

## Project Summary: Data Extraction, Transformation, Loading (ETL) and Querying Tool

This project provides a set of functions for performing ETL operations on a dataset and querying a SQLite database. I utilized GitHub Copilot to convert my previous Python code into Rust, modifying it as necessary to improve accuracy and ensure proper error handling.

### YouTube Video Link
[YouTube Link](https://youtu.be/eYr2D0u7P-4?si=jsTaQMgiSqfkWXVe)

## Components:

### Data Extraction:
- The `extract` function downloads data from a specified URL and saves it to a local file.

### Data Transformation and Loading:
- The `transform_load` function reads a CSV dataset and inserts its records into a SQLite database after performing necessary table operations. It creates a table named `WRRankingDB` with specific columns for ranking, player name, team, opponent, matchup, start/sit status, and projected fantasy points.

### Database Querying:
- The `query` function allows users to perform CRUD operations on the database. It logs the queries into a Markdown file named `query_log.md`.

### Logging:
- The `log_query` function appends SQL queries to a log file, facilitating tracking and analysis of executed queries.

## Preparation and Dependency Installation:
1. Open codespaces.
2. Wait for codespaces to be built.
3. Build dependencies: `cargo build`.
4. Extract data: `cargo run extract`.
5. Transform and load data: `cargo run transform_load`.
6. Sample query: You can use `cargo run query <insert your own query here>` for CRUD operations.

## Check Format and Test Errors:
- Format code: `make format`.
- Lint code: `make lint`.
- Test code: `make test`.

## Use of LLM
In my coding process, I utilized a Large Language Model (LLM) to assist with converting existing Python code to Rust. The LLM helped me generate Rust functions that aligned with the syntax and structure of Rust programming, while also providing suggestions for error handling and optimizing code efficiency. It served as a valuable resource in understanding Rust's idioms and best practices, ultimately enhancing the quality of my code.

