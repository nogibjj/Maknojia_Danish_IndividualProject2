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

# Example: Extract data
extract: 
	cargo run extract

# Example: Transform and Load data
transform_load:
	cargo run transform_load

# Example: Create a database entry
create:
	cargo run query "INSERT INTO WRRankingDB (RK, 'PLAYER NAME', TEAM, OPP, MATCHUP, 'START/SIT', 'PROJ. FPTS') VALUES (600, 'John Doe', 'Team A', 'Team B', 'Favorable', 'Start', 12.3);"
	
# Example: Read from the database
read:
	cargo run query "SELECT * FROM WRRankingDB WHERE server = 'John Doe';"

# Example: Update a database entry
update:
	cargo run update "UPDATE WRRankingDB SET 'PLAYER NAME' = 'Jane Doe', TEAM = 'New Team', OPP = 'New Opp', MATCHUP = 'New Matchup', 'START/SIT' = 'Start', 'PROJ. FPTS' = 20.5 WHERE id = 1;"


# Example: Delete a database entry
delete:
	cargo run delete "DELETE FROM WRRankingDB WHERE id = 1;"


