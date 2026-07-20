# tests/utils.py
import base64
import mimetypes
import os


def to_data_uri(path: str) -> str:
    if not os.path.isfile(path):
        raise FileNotFoundError(f"Image file not found: {path}")
    mime_type, _ = mimetypes.guess_type(path)
    mime_type = mime_type or "image/png"
    with open(path, "rb") as f:
        encoded = base64.b64encode(f.read()).decode("ascii")
    return f"data:{mime_type};base64,{encoded}"


def get_assets_dir() -> str:
    path1 = os.path.abspath(os.path.join(os.path.dirname(__file__), "../assets"))
    if os.path.isdir(path1):
        return path1
    path2 = os.path.abspath("assets")
    if os.path.isdir(path2):
        return path2
    raise FileNotFoundError("Could not find the 'assets' directory.")


def get_logo_uri() -> str:
    return to_data_uri(os.path.join(get_assets_dir(), "images", "logo.png"))
