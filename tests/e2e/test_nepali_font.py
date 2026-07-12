# tests/e2e/test_nepali_font.py
import pytest


@pytest.mark.parametrize("output_format", ["pdf", "html"])
def test_nepali_font_rendering(api_client, save_file, output_format):
    """Tests Devanagari font rendering with inline font switching."""
    template_id = "nepali_font"

    payload = {
        "template_id": template_id,
        "format": output_format,
        "data": {"name_np": "अमृत गिरी", "address_np": "काठमाडौं, नेपाल"},
        "page": {"default_font": "Noto Sans Devanagari"},
        "content": [
            {"type": "heading", "level": 1, "content": [{"text": "नमस्ते (Hello)"}]},
            {
                "type": "paragraph",
                "content": [
                    {"text": "यस पत्रको उद्देश्य "},
                    {"key": "name_np"},
                    {"text": " लाई सम्बोधन गर्नु हो।"},
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {"text": "Customer Name: ", "font_family": "Times New Roman"},
                    {"key": "name_np"},
                ],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
