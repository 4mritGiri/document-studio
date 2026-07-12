# tests/e2e/test_letterhead_invoice.py

import pytest

from tests.utils import get_logo_uri


@pytest.mark.parametrize("output_format", ["pdf", "html"])
def test_letterhead_invoice(api_client, save_file, output_format):
    """Tests Devanagari font rendering with inline font switching."""
    template_id = "letterhead_invoice"
    payload = {
        "template_id": template_id,
        "format": output_format,
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
                        "src": get_logo_uri(),
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
            {
                "type": "paragraph",
                "content": [{"text": "Bill To: "}, {"key": "bill_to"}],
            },
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
                    [
                        {
                            "type": "paragraph",
                            "content": [{"text": "Kathmandu-32, Nepal"}],
                        }
                    ],
                ],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
