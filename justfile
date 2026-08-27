set shell := ["bash", "-uc"]
python_version := "3.13"
venv := ".venv"
python := venv + "/bin/python"

# List of all commands
default:
    @just --list --unsorted \
        --list-heading $'\033[1mAVIN DEV COMMANDS\033[0m\n\n' \
        --list-prefix '  → '

# ────────────────────────────────────────────────────────────────────────────
# Environment
# ────────────────────────────────────────────────────────────────────────────

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

# ────────────────────────────────────────────────────────────────────────────
# Code quality
# ────────────────────────────────────────────────────────────────────────────

# Format & lint rust code
[group('Code quality')]
check-rs:
    cargo fmt --all
    cargo clippy

# Format, lint, typecheck python code
[group('Code quality')]
check-py:
    uv run ruff format
    uv run ruff check --select I --fix
    uv run ruff check
    uv run mypy

# ────────────────────────────────────────────────────────────────────────────
# Tests
# ────────────────────────────────────────────────────────────────────────────

# Run rust unit tests
[group('Tests')]
test-rs:
    cargo test --jobs 4

# Run tests for one Rust module
[group('Tests')]
test-rs-mod crate mod:
    cargo test -p {{crate}} {{mod}}

# Run rust ignored tests
[group('Tests')]
test-rs-ignored:
	cargo test --lib --jobs 4 -- --ignored

# Run python unit tests
[group('Tests')]
test-py:
    uv run pytest -m "not ignored"
    uv run pytest python/avin --doctest-modules

# Run one python test file with stdout
[group('Tests')]
test-py-file file:
    uv run pytest "{{file}}" -s

# Run python ignored tests
[group('Tests')]
test-py-ignored:
    uv run pytest -m ignored

# ────────────────────────────────────────────────────────────────────────────
# Project
# ────────────────────────────────────────────────────────────────────────────

# Check rust code
[group('Project')]
rs:
    just check-rs
    just test-rs

# Check python code
[group('Project')]
py:
    just check-py
    just test-py

# Maturin build
[group('Project')]
maturin:
    uv run maturin develop

# Fix imports, format, lint, typecheck and test
[group('Project')]
pre-commit:
    just rs
    just build
    just py

# Build AVIN Book and open in browser
[group('Project')]
book:
    mdbook build docs/avin_book --open

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

# Remove caches and .venv
[group('Project')]
clean-all:
    just clean
    rm -rf {{venv}}/

# Create avin.zip from HEAD
[group('Project')]
archive:
    git archive --format zip HEAD -o avin.zip


import? 'local.justfile'
