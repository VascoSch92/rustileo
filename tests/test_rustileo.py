import rustileo
from pathlib import Path
import re


def test_version():
    assert rustileo.__version__ == _get_rustileo_version()


def _get_rustileo_version() -> str:
    # Path to the Cargo.toml file
    cargo_toml_path = Path("Cargo.toml")

    # Check if the file exists
    if not cargo_toml_path.exists():
        raise FileNotFoundError("Cargo.toml not found.")

    # Read the contents of the file
    with open(cargo_toml_path, "r", encoding="utf-8") as file:
        for line in file:
            match = re.match(r"^\s*version\s*=\s*[\"'](.+?)[\"']\s*$", line)
            if match:
                return match.group(1)
    raise ValueError("Version not found in Cargo.toml.")
