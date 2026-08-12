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

# Run unit tests
[group('Tests')]
test:
	cargo test --lib --jobs 4

# Run doc tests
[group('Tests')]
test-doc:
	cargo test --doc --jobs 4

# ----------------------------------------------------------------------------
# Project
# ----------------------------------------------------------------------------

# Fix imports, format, lint, typecheck and test
[group('Project')]
pre-commit:
    just test
    just test-doc

# Create avin.zip from HEAD
[group('Project')]
run:
    cargo run -p avin --bin scratch

# Create avin.zip from HEAD
[group('Project')]
archive:
    git archive --format zip HEAD -o avin.zip
