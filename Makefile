.DEFAULT_GOAL:=help
SHELL=bash
PYTHONPATH=
VENV=.venv
PY=avin_data_py
PY_ENV=source .venv/bin/activate && cd avin_data_py
PY_APP=~/.local/bin/avin-data

.venv: ## Create python virtual environment & install requirements
	python3 -m venv $(VENV)
	$(MAKE) requirements

requirements: .venv ## Install/Update Python project requirements
	$(VENV)/bin/python -m pip install --upgrade pip
	$(VENV)/bin/python -m pip install --upgrade -r $(PY)/requirements.txt

dev: .venv  ## Activate venv & start neovim for this project
	source .venv/bin/activate
	nvim -c AvinDev

check: ## Run ruff, mypy clippy
	ruff check --select I --fix
	mypy $(PY) --no-namespace-packages
	cargo clippy

fix: ## Automatically apply lint suggestions
	cargo clippy --fix

fmt: ## Run ruff format & cargo fmt
	cargo fmt --all
	ruff format

test: ## Run pytests, lib-tests, doc-tests
	$(PY_ENV) && pytest tests
	cargo test --lib --jobs 4 -- --test-threads=1
	cargo test --doc --jobs 4 -- --test-threads=1

test_ignored: ## Run slow ingnored tests
	cargo test --lib --jobs 4 -- --ignored --test-threads=1

pre-commit: ## Make check, fmt, test
	$(MAKE) check
	$(MAKE) fmt
	$(MAKE) test

build: .venv ## Build the project
	$(PY_ENV) && flit build --no-use-vcs
	$(PY_ENV) && pyinstaller avin_data/cli.py \
		--onefile \
		--specpath build \
		--name avin-data
	cargo build --jobs 4

publish: ## Publish PyPl & crates.io
	source .venv/bin/activate && cd $(PY) && flit publish
	cargo publish -p avin_utils
	cargo publish -p avin_core
	cargo publish -p avin_connect
	cargo publish -p avin_data
	cargo publish -p avin_analyse
	cargo publish -p avin_scanner
	cargo publish -p avin_simulator
	cargo publish -p avin_strategy
	cargo publish -p avin_tester
	cargo publish -p avin_trader
	cargo publish -p avin_terminal
	cargo publish -p avin_adviser
	cargo publish -p avin_gui
	cargo publish -p avin

install: build ## Install the project
	$(PY_ENV) && flit install
	rm -rf $(PY_APP)
	install -Dm755 $(PY)/dist/avin-data $(PY_APP)
	install -Dm644 res/config.toml ~/.config/avin/config.toml

doc: build ## Create and open local documentation
	cargo doc --workspace --open --no-deps --color always --jobs 4

clean: ## Clean up caches, build artifacts, and the venv
	rm -rf .mypy_cache/
	rm -rf .pytest_cache/
	rm -rf .ruff_cache/
	rm -rf .venv/
	rm -rf $(PY)/build
	rm -rf $(PY)/dist
	ruff clean
	cargo clean

run:
	cargo run --bin a-aaa --jobs 4  ## Run temp bin
data:
	cargo run --bin avin-data --jobs 4 --release
adviser:
	cargo run --bin avin-adviser --jobs 4 --release
analyse:
	cargo run --bin avin-analyse --jobs 4 --release
backscan:
	cargo run --bin avin-backscan --jobs 4 --release
backtest:
	cargo run --bin avin-backtest --jobs 4 --release
scanner:
	cargo run --bin avin-scanner --jobs 4 --release
simulator:
	cargo run --bin avin-simulator --jobs 4 --release
tester:
	cargo run --bin avin-tester --jobs 4 --release
trader:
	cargo run --bin avin-trader --jobs 4 --release
terminal:
	cargo run --bin avin-terminal --jobs 4 --release

T1="\033[1m"
T2="\033[0m"
B0="\033[32m"
B1="    \033[32m"
B2="\033[0m"
help:
	@echo -e $(T1)Usage:$(T2) make [$(B0)target$(B2)]
	@echo ""
	@echo -e $(T1)Virtual environment:$(T2)
	@echo -e $(B1).venv$(B2)"          Create python .venv"
	@echo -e $(B1)requirements$(B2)"   Install/Update python requirements"
	@echo -e $(B1)dev$(B2)"            Activate venv & start neovim"
	@echo ""
	@echo -e $(T1)Code quality:$(T2)
	@echo -e $(B1)check$(B2)"          Linting ruff, mypy, clippy"
	@echo -e $(B1)fix$(B2)"            Auto apply linting suggestions"
	@echo -e $(B1)fmt$(B2)"            Autoformatting"
	@echo -e $(B1)test$(B2)"           Run pytests, lib-tests, doc-tests"
	@echo -e $(B1)test_ignored$(B2)"   Run slow ignored tests"
	@echo -e $(B1)pre-commit$(B2)"     Make all code quality"
	@echo ""
	@echo -e $(T1)Build project:$(T2)
	@echo -e $(B1)build$(B2)"          Build python and rust sources"
	@echo -e $(B1)publish$(B2)"        Publish package pypi.org & crates.io"
	@echo -e $(B1)install$(B2)"        Install the project"
	@echo -e $(B1)doc$(B2)"            Create and open local documentation"
	@echo -e $(B1)clean$(B2)"          Clean the project"
	@echo ""
	@echo -e $(T1)Run:$(T2)
	@echo -e $(B1)run$(B2)"            Run temp bin aaa"
	@echo -e $(B1)analyse$(B2)"        Run analyse"
	@echo -e $(B1)backscan$(B2)"       Run backscan"
	@echo -e $(B1)backtest$(B2)"       Run backtest"
	@echo -e $(B1)data$(B2)"       	   Run data"
	@echo -e $(B1)scanner$(B2)"        Run scanner"
	@echo -e $(B1)simulator$(B2)"      Run scanner"
	@echo -e $(B1)tester$(B2)"         Run tester"
	@echo -e $(B1)trader$(B2)"         Run trader"
	@echo -e $(B1)terminal$(B2)"       Run terminal"
	@echo ""
	@echo -e $(B1)help$(B2)"           Display this help message"

# Each entry of .PHONY is a target that is not a file
.PHONY: check, fmt, test, pre-commit, build, install, publish, clean
.PHONY: requirements, dev, run, help, test_ignored
.PHONY: analyse, data, backscan, backtest, scanner, tester, trader, terminal

