# tests/e2e/test_qr_verification.py
import pytest


@pytest.mark.parametrize("output_format", ["pdf", "html"])
def test_qr_code_generation(api_client, save_file, output_format):
    """Tests QR code generation in both PDF and HTML formats."""
    template_id = "loan_sanction_qr"
    verification_url = "https://verify.siddharthabank.com/doc/LOAN-2026-0091?hash=a8f5f167f44f4964e6c99822b8b46cd3"

    payload = {
        "template_id": template_id,
        "format": output_format,
        "data": {"customer_name": "Amrit Giri", "loan_amount": "NPR 5,00,000"},
        "content": [
            {"type": "spacer", "height": "2cm"},
            {
                "type": "qr_code",
                "data": verification_url,
                "width": "3cm",
                "alignment": "center",
            },
            {
                "type": "paragraph",
                "alignment": "center",
                "content": [{"text": "Scan to verify", "italic": True}],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None, "Response content is empty"

    save_file(f"{template_id}.{output_format}", response.content)
