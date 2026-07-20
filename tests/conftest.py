# tests/conftest.py
import os
import sys

import pytest

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
from tests.client import APIClient


@pytest.fixture(scope="session")
def api_client():
    api_key = os.environ.get(
        "DOCUMENT_ENGINE_API_KEY",
        "90de5ac5c9501019b9d3ae98f7503d82a5e1de1a7c49213fc3559c91f7b01a56",
    )
    return APIClient(base_url="http://localhost:3000/api/v1/", api_key=api_key)


@pytest.fixture(scope="session")
def output_dir():
    project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    path = os.path.join(project_root, "docs", "examples")
    os.makedirs(path, exist_ok=True)
    print(f"\n📂 Output directory: {path}")
    return path


@pytest.fixture
def save_file(output_dir):
    def _save(filename: str, content: bytes) -> str:
        file_path = os.path.join(output_dir, filename)
        with open(file_path, "wb") as f:
            f.write(content)
        print(f"✅ Saved: {file_path}")
        return file_path

    return _save
