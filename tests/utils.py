# tests/utils.py
import base64
import mimetypes
import os


def to_data_uri(path: str) -> str:
    """Converts a local image file to a base64 data URI."""
    # Safety check: ensure we are opening a file, not a directory
    if not os.path.isfile(path):
        raise FileNotFoundError(f"Image file not found: {path}")

    mime_type, _ = mimetypes.guess_type(path)
    mime_type = mime_type or "image/png"
    with open(path, "rb") as f:
        encoded = base64.b64encode(f.read()).decode("ascii")
    return f"data:{mime_type};base64,{encoded}"


def get_assets_dir() -> str:
    """Helper to get the absolute path to the assets directory."""
    # 1. Try relative to this file (tests/utils.py -> ../assets)
    path1 = os.path.abspath(os.path.join(os.path.dirname(__file__), "../assets"))
    if os.path.isdir(path1):
        return path1

    # 2. Try relative to CWD (./assets)
    path2 = os.path.abspath("assets")
    if os.path.isdir(path2):
        return path2

    raise FileNotFoundError("Could not find the 'assets' directory.")


def get_logo_uri() -> str:
    """Helper to get the standard project logo as a data URI."""
    # 1. Get the directory path
    assets_dir = get_assets_dir()
    # 2. Safely join the filename
    logo_path = os.path.join(assets_dir, "images", "logo.png")

    # 3. Convert the FILE to base64
    return to_data_uri(logo_path)
