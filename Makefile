# Minimal Makefile for development

# variables
# you can set the first variable from the environment
PYPI_PASSWORD   ?=
TESTSDIR        = tests

# It is first such that "make" without argument is like "make help".
help:
	@echo "[HELP] Makefile commands:"
	@echo " * init: initialize venv"
	@echo " * init-dev: install dev dependencies"
	@echo " * init-lint: install lint dependencies"
	@echo " * init-test: install tests dependencies"
	@echo " * fastbuild: Fast Build and Tests"
	@echo " * lint: Formatting and checking for both Rust and Python"
	@echo " * rust-lint: Formatting and checking for Rust"
	@echo " * python-lint: Formatting and checking for Python"
	@echo " * test: run tests"

.PHONY: help Makefile

init:
	@echo "[INFO] initialize venv"
	@rm -rf .venv
	@uv venv
	@uv sync
	@uv pip list

init-dev:
	@echo "[INFO] Install dev dependencies"
	@uv sync --all-groups
	@uv pip list

init-lint:
	@echo "[INFO] Install lint dependencies"
	@uv sync --group lint
	@uv pip list

init-test:
	@echo "[INFO] Install tests dependencies"
	@uv sync --group test
	@uv pip list

fastbuild:
	@echo "[INFO] Fast Build and Tests"
	@echo "[INFO] Delete target directory"
	@rm -rf target
	@echo "[INFO] Run maturin develop"
	@uv run maturin develop
	@echo "[INFO] Uninstall rustileo from env"
	@uv pip uninstall rustileo
	@echo "[INFO] Clean uv cache"
	@uv cache clean
	@echo "[INFO] Run test"
	@uv run pytest .

lint:
	@echo "[INFO] Formatting and checking for both Rust and Python"
	@make lint-python
	@make lint-rust

lint-rust:
	@echo "[INFO] Formatting and checking for Rust"
	@cargo fmt
	@cargo clippy

lint-python:
	@echo "[INFO] Formatting and checking for Python"
	@uv run ruff check . --fix
	@uv run ruff format .

test:
	@echo "[INFO] Run tests"
	@uv run pytest .
