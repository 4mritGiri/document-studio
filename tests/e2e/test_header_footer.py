import base64
import mimetypes
import os

import requests


def to_data_uri(path: str) -> str:
    mime_type, _ = mimetypes.guess_type(path)
    mime_type = mime_type or "image/png"
    with open(path, "rb") as f:
        encoded = base64.b64encode(f.read()).decode("ascii")
    return f"data:{mime_type};base64,{encoded}"


url = "http://localhost:3000/api/v1/generate"

LOGO_PATH = "/media/amrit/SSDAmrit/Builds/local/DocumentStudio/assets/images/logo.png"
OUTPUT_DIR = "docs/examples/"
PLACEHOLDER_LOGO = to_data_uri(LOGO_PATH)
payload = {
    "template_id": "header_footer_demo",
    "data": {"company_name": "Fintech Nepal Pvt Ltd."},
    "page": {
        "header": {
            # Shown on every page except the first (Word's "Different First Page")
            "skip_first_page": True,
            "alignment": "right",
            "content": [{"key": "company_name"}, {"text": " - Confidential"}],
        },
        "footer": {
            # Shown on every page, including the first
            "alignment": "center",
            "content": [{"page_number": True, "format": "Page {current} of {total}"}],
        },
    },
    "content": [
        {"type": "heading", "level": 1, "content": [{"text": "Cover Page"}]},
        {
            "type": "paragraph",
            "content": [{"text": "No header shown here (skip_first_page)."}],
        },
        {"type": "page_break"},
        {"type": "heading", "level": 1, "content": [{"text": "Page 2"}]},
        {"type": "paragraph", "content": [{"text": "Header now appears at the top."}]},
        {"type": "page_break"},
        {"type": "heading", "level": 1, "content": [{"text": "Page 3"}]},
        {"type": "paragraph", "content": [{"text": "Header/footer keep repeating."}]},
    ],
}

api_key = os.environ.get(
    "DOCUMENT_ENGINE_API_KEY",
    "90de5ac5c9501019b9d3ae98f7503d82a5e1de1a7c49213fc3559c91f7b01a56",
)
headers = {"x-api-key": f"{api_key}"}
response = requests.post(url, json=payload, headers=headers)

if response.status_code == 200:
    with open(OUTPUT_DIR + "header_footer_demo.pdf", "wb") as f:
        f.write(response.content)
    print("✅ Success! 'header_footer_demo.pdf' has been generated.")
else:
    print(f"❌ Error: {response.status_code}")
    print(response.text)
