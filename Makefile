.DEFAULT_GOAL := help
SHELL=bash
PYTHONPATH=
VENV=.venv
VENV_BIN=$(VENV)/bin
AVIN_DATA_APP=~/.local/bin/avin-data

.venv: ## Create python virtual environment & install requirements
	python3 -m venv $(VENV)
	$(MAKE) requirements

requirements: .venv ## Install/Update Python project requirements
	$(VENV_BIN)/python -m pip install --upgrade pip
	$(VENV_BIN)/python -m pip install --upgrade -r avin_data/requirements.txt

check: ## Run ruff, mypy clippy
	ruff check --select I --fix
	mypy avin_data --no-namespace-packages
	cargo clippy

fmt: ## Run ruff format & cargo fmt
	cargo fmt --all
	ruff format

test: ## Run pytests, lib-tests, doc-tests
	source .venv/bin/activate && cd avin_data && pytest tests
	cargo test --lib
	cargo test --doc

test_ignored: ## Run slow ingnored tests
	cargo test --lib -- --ignored

pre-commit: ## Make check, fmt, test
	$(MAKE) check
	$(MAKE) fmt
	$(MAKE) test

build: .venv ## Build the project
	cd avin_data && flit build --no-use-vcs
	cd avin_data && pyinstaller cli.py \
		--onefile \
		--specpath build \
		--name avin-data
	cargo build --jobs 2

publish: ## Publish PyPl & crates.io
	cd avin_data && flit publish
	cargo publish -p avin_utils
	cargo publish -p avin_core
	cargo publish -p avin_analyse
	cargo publish -p avin_strategy
	cargo publish -p avin_tester
	cargo publish -p avin_connect
	cargo publish -p avin_trader
	cargo publish -p avin

install: build ## Install the project
	cd avin_data && flit install
	rm -rf $(AVIN_DATA_APP)
	install -Dm755 avin_data/dist/avin-data $(AVIN_DATA_APP)

doc: build ## Create and open local documentation
	cargo doc --workspace --open --no-deps --color always --jobs 2

clean: ## Clean up caches, build artifacts, and the venv
	rm -rf .mypy_cache/
	rm -rf .pytest_cache/
	rm -rf .ruff_cache/
	rm -rf .venv/
	rm -rf avin_data/build
	rm -rf avin_data/dist
	ruff clean
	cargo clean

dev: .venv  ## Activate venv & start neovim for this project
	source .venv/bin/activate
	nvim -c AvinDev

r: ## Run temp bin (gitignored main.rs)
	cargo run --bin avin --jobs 2

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
	@echo ""
	@echo -e $(T1)Code quality:$(T2)
	@echo -e $(B1)check$(B2)"          Linting ruff, mypy, clippy"
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
	@echo -e $(T1)Other:$(T2)
	@echo -e $(B1)dev$(B2)"            Activate venv & start neovim"
	@echo -e $(B1)r$(B2)"              Run temp bin (gitignored main.rs)"
	@echo -e $(B1)help$(B2)"           Display this help message"

# help: ## Display this help screen
# 	@echo -e "\033[1mUsage:\033[0m make [target]"
# 	@echo ""
# 	@echo -e "\033[1mAvailable targets:\033[0m"
# 	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}' | sort


# Each entry of .PHONY is a target that is not a file
.PHONY: check, fmt, test, pre-commit, build, install, publish, clean
.PHONY: requirements, dev, r, help, test_ignored
