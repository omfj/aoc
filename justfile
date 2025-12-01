# Generate a new day
generate YEAR DAY:
    @cargo run --quiet -- generate --year {{YEAR}} --day {{DAY}}

# Run a day
run YEAR DAY:
    @cargo run --quiet --release -- run --year {{YEAR}} --day {{DAY}}

# Run tests
test:
    @cargo test --release

# Fetch the input for a day
fetch YEAR DAY:
    @cargo run --quiet -- fetch --year {{YEAR}} --day {{DAY}}
