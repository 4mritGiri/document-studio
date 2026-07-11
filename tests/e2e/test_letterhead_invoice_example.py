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
    "template_id": "letterhead_invoice_demo",
    "data": {
        "company_name": "Fintech Nepal Pvt Ltd.",
        "invoice_no": "INV-2026-0091",
        "invoice_date": "July 10, 2026",
        "bill_to": "Siddhartha Bank Ltd., Naxal, Kathmandu",
        "items": [
            {
                "desc": "MLMS - Implementation",
                "qty": "1",
                "unit_price": "3,50,000",
                "total": "3,50,000",
            },
            {
                "desc": "Annual Support & Maintenance",
                "qty": "1",
                "unit_price": "75,000",
                "total": "75,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
            {
                "desc": "Staff Training (per session)",
                "qty": "3",
                "unit_price": "10,000",
                "total": "30,000",
            },
        ],
        "grand_total": "NPR 4,55,000",
    },
    "page": {
        # This is the reusable "letterhead" — it's drawn behind every page,
        # the same way a Word letterhead template works.
        "background": [
            # Solid diagonal cut in the top-left corner
            {
                "type": "placed",
                "anchor": "top-left",
                "content": {
                    "type": "shape",
                    "kind": "triangle",
                    "width": "6cm",
                    "height": "3cm",
                    "fill": "#1a1a1a",
                },
            },
            # Smaller accent triangle bottom-right
            {
                "type": "placed",
                "anchor": "bottom-right",
                "content": {
                    "type": "shape",
                    "kind": "triangle",
                    "width": "4cm",
                    "height": "2cm",
                    "fill": "#1a1a1a",
                    "rotate": "180deg",
                },
            },
            # Logo pinned top-left, on top of the letterhead
            {
                "type": "placed",
                "anchor": "top-left",
                "dx": "2cm",
                "dy": "0.8cm",
                "content": {
                    "type": "image",
                    "src": PLACEHOLDER_LOGO,
                    "width": "2.5cm",
                },
            },
        ],
        "header": {
            "skip_first_page": True,
            "alignment": "right",
            "content": [{"key": "company_name"}, {"text": " - Invoice"}],
        },
        "footer": {
            # Icon + text contact bar, like the reference letterhead's footer strip
            "content": [
                {"text": ""}
            ],  # unused when we render a Columns node below instead
        },
    },
    "content": [
        {"type": "spacer", "height": "2.5cm"},  # clear the letterhead graphics
        {
            "type": "heading",
            "level": 1,
            "alignment": "center",
            "content": [{"text": "INVOICE"}],
        },
        {
            "type": "paragraph",
            "content": [
                {"text": "Invoice No: "},
                {"key": "invoice_no"},
                {"text": "   |   Date: "},
                {"key": "invoice_date"},
            ],
        },
        {"type": "paragraph", "content": [{"text": "Bill To: "}, {"key": "bill_to"}]},
        {"type": "spacer", "height": "0.5cm"},
        {
            "type": "table",
            "headers": ["Description", "Qty", "Unit Price", "Total"],
            "loop_data": "items",
            "row_template": [
                {"key": "desc"},
                {"key": "qty"},
                {"key": "unit_price"},
                {"key": "total"},
            ],
            "footer": [
                {"text": ""},
                {"text": ""},
                {"text": "Grand Total:", "bold": True},
                {"key": "grand_total", "bold": True},
            ],
            "style": {
                "width": "100%",
                "repeat_header": True,
                "columns": ["3fr", "1fr", "1.5fr", "1.5fr"],
                "header_bg": "#1f2937",
                "column_align": ["left", "right", "right", "right"],
            },
        },
        {"type": "spacer", "height": "1.5cm"},
        {
            "type": "columns",
            "column_widths": ["auto", "1fr"],
            "gutter": "0.4em",
            "items": [
                [{"type": "paragraph", "content": [{"text": "📍"}]}],
                [{"type": "paragraph", "content": [{"text": "Kathmandu-32, Nepal"}]}],
            ],
        },
    ],
}

api_key = os.environ.get(
    "DOCUMENT_ENGINE_API_KEY",
    "90de5ac5c9501019b9d3ae98f7503d82a5e1de1a7c49213fc3559c91f7b01a56",
)
headers = {"x-api-key": f"{api_key}"}
response = requests.post(url, json=payload, headers=headers)

if response.status_code == 200:
    with open(OUTPUT_DIR + "letterhead_invoice_demo.pdf", "wb") as f:
        f.write(response.content)
    print("✅ Success! 'letterhead_invoice_demo.pdf' has been generated.")
else:
    print(f"❌ Error: {response.status_code}")
    print(response.text)
