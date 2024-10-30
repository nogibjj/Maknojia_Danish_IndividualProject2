# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Extract data
extract: 
	cargo run extract

# Transform and Load data
transform_load:
	cargo run transform_load

# Create a database entry
create:
	cargo run query "INSERT INTO WRRankingDB (rk, player_name, team, opp, matchup, start_sit, proj_fpts) VALUES (1, 'Danish Maknojia', 'Team Alpha', 'Team Beta', 'Matchup X', 'Start', 15.7);"

# Read from the database
read:
	cargo run query "SELECT * FROM WRRankingDB WHERE team = 'Team Alpha';"

# Update a database entry
update:
	cargo run query "UPDATE WRRankingDB SET proj_fpts = 18.5 WHERE player_name = 'Danish Maknojia' AND matchup = 'Matchup X';"

# Delete a database entry
delete:
	cargo run query "DELETE FROM WRRankingDB WHERE player_name = 'Danish Maknojia' AND team = 'Team Alpha';"


