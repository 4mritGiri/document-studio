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
                # FIX 1: Ensure headers match the number of columns (6 total)
                "headers": [
                    "Description",
                    "Qty",
                    "Price",
                    "Total",
                    "VAT (13%)",
                    "Net Total",
                ],
                "loop_data": "items",
                "row_template": [
                    {"key": "desc"},
                    {"key": "qty"},
                    {"key": "price"},
                    # INLINE MATH: =qty * price
                    {"formula": "=qty * price", "bold": True},
                    # PERCENTAGE MATH: =qty * price * 0.13
                    {"formula": "=qty * price * 0.13", "format": "NPR {value}"},
                    # COMPLEX MATH: Parentheses and multiple operators
                    {
                        "formula": "=(qty * price) - (qty * price * 0.13)",
                        "format": "{value}",
                    },
                ],
                "footer": [
                    # FIX 2: colspan=4 spans the first 4 columns (Desc, Qty, Price, Total)
                    {"text": "FINANCIAL SUMMARY", "bold": True, "colspan": 4},
                    # This occupies column 5
                    {
                        "formula": "=sum(price) * 0.13",
                        "format": "{value}",
                        "bold": True,
                    },
                    # This occupies column 6 (4 + 1 + 1 = 6 total columns)
                    {
                        "formula": "=sum(price) * 1.13",
                        "format": "{value}",
                        "bold": True,
                    },
                ],
                "style": {
                    "width": "100%",
                    # Provide exactly 6 column definitions to match the headers
                    "columns": ["3fr", "1fr", "1.5fr", "1.5fr", "1.5fr", "1.5fr"],
                    # FIX 4: Provide exactly 6 alignments
                    "column_align": [
                        "left",
                        "right",
                        "right",
                        "right",
                        "right",
                        "right",
                    ],
                    "header_bg": "#1f2937",
                },
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"

    save_file("advanced_invoice_math.pdf", response.content)
