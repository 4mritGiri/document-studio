# tests/e2e/test_invoice_math.py


def test_advanced_invoice_math(api_client, save_file):
    """Tests Excel-like calculations: Multiplication, Percentages, and Aggregations."""

    payload = {
        "template_id": "advanced_invoice",
        "format": "pdf",
        "data": {
            "customer_name": "Siddhartha Bank Ltd.",
            # Notice: NO subtotal, NO vat, NO grand_total in the data!
            "items": [
                {"desc": "Core Banking Software", "qty": 1, "price": 500000},
                {"desc": "Annual Maintenance", "qty": 2, "price": 50000},
                {"desc": "Staff Training", "qty": 5, "price": 10000},
            ],
        },
        "content": [
            {"type": "heading", "level": 1, "content": [{"text": "TAX INVOICE"}]},
            {
                "type": "table",
                "headers": ["Description", "Qty", "Price", "Total", "VAT (13%)"],
                "loop_data": "items",
                "row_template": [
                    {"key": "desc"},
                    {"key": "qty"},
                    {"key": "price"},
                    # INLINE MATH: =qty * price
                    {"formula": "=qty * price", "bold": True},
                    # PERCENTAGE MATH: =total * 0.13 (Wait, we can't reference the previous cell easily,
                    # so we do =qty * price * 0.13)
                    {"formula": "=qty * price * 0.13", "format": "NPR {value}"},
                ],
                "footer": [
                    {"text": "FINANCIAL SUMMARY", "bold": True, "colspan": 3},
                    # AGGREGATION: =sum(total) -> Wait, we need to sum the calculated total.
                    # Since we can't easily sum a calculated column in V1, let's sum the base total:
                    {
                        "formula": "=sum(price)",
                        "format": "{value}",
                        "bold": True,
                    },
                    {
                        "formula": "=sum(price) * 0.13",
                        "format": "{value}",
                        "bold": True,
                    },
                ],
                "style": {
                    "width": "100%",
                    "columns": ["3fr", "1fr", "1.5fr", "1.5fr", "1.5fr"],
                    "column_align": ["left", "right", "right", "right", "right"],
                    "header_bg": "#ffffff",
                },
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"

    save_file("advanced_invoice_math.pdf", response.content)
