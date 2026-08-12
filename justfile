set shell := ["bash", "-uc"]
python_version := "3.13"
venv := ".venv"
python := venv + "/bin/python"

# List of all commands
default:
    @just --list --unsorted \
        --list-heading $'\033[1mAVIN DEV COMMANDS\033[0m\n\n' \
        --list-prefix '  → '

# ----------------------------------------------------------------------------
# Tests
# ----------------------------------------------------------------------------

# Run rust unit tests
[group('Tests')]
test-rs:
	cargo test --lib --jobs 4

# Run doc tests
[group('Tests')]
test-rs-doc:
	cargo test --doc --jobs 4

# Run python unit tests
[group('Tests')]
test-py:
    uv run pytest -m "not integration and not slow"

# ----------------------------------------------------------------------------
# Project
# ----------------------------------------------------------------------------

# Fix imports, format, lint, typecheck and test
[group('Project')]
pre-commit:
    just test-rs
    just test-doc
    just test-py

[group('Project')]
build:
    maturin develop

# Run avin scratch
[group('Project')]
run:
    cargo run -p avin --bin scratch

# Create avin.zip from HEAD
[group('Project')]
archive:
    git archive --format zip HEAD -o avin.zip
