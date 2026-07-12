# tests/e2e/test_header_footer.py

import pytest


@pytest.mark.parametrize("output_format", ["pdf", "html"])
def test_header_footer(api_client, save_file, output_format):
    """Tests Devanagari font rendering with inline font switching."""
    template_id = "header_footer"
    payload = {
        "template_id": template_id,
        "format": output_format,
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
                "content": [
                    {"page_number": True, "format": "Page {current} of {total}"}
                ],
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
            {
                "type": "paragraph",
                "content": [{"text": "Header now appears at the top."}],
            },
            {"type": "page_break"},
            {"type": "heading", "level": 1, "content": [{"text": "Page 3"}]},
            {
                "type": "paragraph",
                "content": [{"text": "Header/footer keep repeating."}],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
