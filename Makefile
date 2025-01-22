# Minimal Makefile for development

# variables
# you can set the first variable from the environment
PYPI_PASSWORD   ?=
TESTSDIR        = tests

# It is first such that "make" without argument is like "make help".
help:
	@echo "[HELP] Makefile commands:"
	@echo " * fastbuild: Fast Build and Tests"
	@echo " * format: Formatting and checking for both Rust and Python"
	@echo " * ruformat: Formatting and checking for Rust"
	@echo " * pyformat: Formatting and checking for Python"
	@echo " * test: run tests"

.PHONY: help Makefile

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

format:
	@echo "[INFO] Formatting and checking for both Rust and Python"
	@make pyformat
	@make ruformat

ruformat:
	@echo "[INFO] Formatting and checking for Rust"
	@cargo fmt
	@cargo clippy

pyformat:
	@echo "[INFO] Formatting and checking for Python"
	@uv run ruff check . --fix
	@uv run ruff format .

test:
	@echo "[INFO] Run tests"
	@uv run pytest .
