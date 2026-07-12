# tests/e2e/test_qr_verification.py
from client import requests

# Simulate a unique verification URL that your Django backend would handle
verification_url = "https://verify.siddharthabank.com/doc/LOAN-2026-0091?hash=a8f5f167f44f4964e6c99822b8b46cd3"

template_id = "loan_sanction_qr"
formats = ["pdf", "html"]

for format in formats:
    payload = {
        "template_id": template_id,
        "format": format,
        "data": {
            "customer_name": "Amrit Giri",
            "loan_amount": "NPR 5,00,000",
        },
        "content": [
            {"type": "spacer", "height": "2cm"},
            # NEW: Add the QR Code Node
            {
                "type": "qr_code",
                "data": verification_url,
                "width": "3cm",
                "alignment": "center",
            },
            {
                "type": "paragraph",
                "alignment": "center",
                "content": [
                    {
                        "text": "Scan to verify document authenticity",
                        "italic": True,
                        "font_family": "Times New Roman",
                    }
                ],
            },
        ],
    }

    response = requests.post(endpoint="/generate", json=payload)

    if response.is_success and response.content is not None:
        with open(f"docs/examples/{template_id}.{format}", "wb") as f:
            f.write(response.content)
        print(
            f"✅ Success! Check '{template_id}.{format}' - the QR code is perfectly crisp!"
        )
    else:
        print(f"❌ Error: {response.text}")
