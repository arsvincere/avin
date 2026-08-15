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
# Environment
# ----------------------------------------------------------------------------

# Create python venv if missing (.venv)
[group('Environment')]
venv:
    @if [ ! -d "{{venv}}" ]; then \
        uv venv --python {{python_version}} {{venv}}; \
    fi

# Install AVIN in editable mode
[group('Environment')]
install: venv
    uv pip install -e .

# Install AVIN with dev dependencies
[group('Environment')]
install-dev: venv
    uv pip install -e ".[dev]"

# ----------------------------------------------------------------------------
# Code quality
# ----------------------------------------------------------------------------

# Fix imports
[group('Code quality')]
check-rs:
    cargo clippy
    cargo fmt --all

# Format code
[group('Code quality')]
check-py:
    uv run ruff check --select I --fix ./python
    uv run ruff check ./python
    uv run ruff format ./python
    uv run mypy ./python

# ----------------------------------------------------------------------------
# Tests
# ----------------------------------------------------------------------------

# Run rust unit tests
[group('Tests')]
test-rs:
	cargo test --lib --jobs 4
	cargo test --doc --jobs 4

# Run rust ignored tests
[group('Tests')]
test-rs-ignored:
	cargo test --lib --jobs 4 -- --ignored

# Run python unit tests
[group('Tests')]
test-py:
    uv run pytest -m "not ignored"

# Run python ignored tests
[group('Tests')]
test-py-ignored:
    uv run pytest -m ignored

# ----------------------------------------------------------------------------
# Project
# ----------------------------------------------------------------------------

# Fix imports, format, lint, typecheck and test
[group('Project')]
pre-commit:
    just check-py
    just check-rs
    just test-rs
    just test-py

[group('Project')]
build:
    maturin develop

# Run avin scratch
[group('Project')]
run:
    cargo run -p avin --bin scratch

# Remove caches
[group('Project')]
clean:
    rm -rf *.egg-info
    rm -rf .cache
    rm -rf .coverage
    rm -rf .mypy_cache
    rm -rf .pytest_cache
    rm -rf .ruff_cache
    rm -rf avin.zip
    rm -rf coverage.xml
    rm -rf htmlcov
    rm -rf build
    rm -rf dist
    rm -rf target
    uv run ruff clean || true

# Create avin.zip from HEAD
[group('Project')]
archive:
    git archive --format zip HEAD -o avin.zip
