# tests/e2e/test_locale_math.py


def test_locale_formatting(api_client, save_file):
    """Tests dynamic locale-aware number formatting."""

    payload = {
        "template_id": "locale_test",
        "format": "pdf",
        "data": {
            "items": [
                {"desc": "Item A", "qty": 1, "price": 1234567.896},
                {"desc": "Item B", "qty": 2, "price": 987654.32},
            ]
        },
        "content": [
            {
                "type": "table",
                "headers": ["Item", "US Style", "Nepali Style", "European Style"],
                "loop_data": "items",
                "row_template": [
                    {"key": "desc"},
                    # 1. International (US): 1,234,567.89
                    {"formula": "=price", "locale": "en-US", "decimal_places": 4},
                    # 2. Nepali/Indian: 12,34,567.89 (Lakhs/Crores)
                    {
                        "formula": "=price",
                        "locale": "en-NP",
                        "format": "NPR {value}",
                        "decimal_places": 4,
                    },
                    # 3. European: 1.234.567,89
                    {
                        "formula": "=price",
                        "locale": "de-DE",
                        "format": "{value} €",
                        "decimal_places": 2,
                    },
                ],
                "footer": [
                    {"text": "Totals", "bold": True},
                    {"formula": "=sum(price)", "locale": "en-US", "bold": True},
                    {
                        "formula": "=sum(price)",
                        "locale": "en-NP",
                        "format": "NPR {value}",
                        "bold": True,
                    },
                    {
                        "formula": "=sum(price)",
                        "locale": "de-DE",
                        "format": "{value} €",
                        "bold": True,
                    },
                ],
            }
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    save_file("locale_formatting.pdf", response.content)
