# tests/e2e/test_pdf_security.py


def test_pdf_encryption(api_client, save_file):
    """Tests AES-256 PDF password protection."""
    template_id = "secure_loan_letter"

    payload = {
        "template_id": template_id,
        "format": "pdf",
        "data": {
            "customer_name": "Amrit Giri",
            "loan_amount": "NPR 5,00,000",
            "account_number": "01234567890123",  # Sensitive PII
        },
        "content": [
            {
                "type": "heading",
                "level": 1,
                "content": [{"text": "CONFIDENTIAL LOAN SANCTION"}],
            },
            {
                "type": "paragraph",
                "content": [
                    {"text": "Dear "},
                    {"key": "customer_name"},
                    {"text": ", your loan of "},
                    {"key": "loan_amount"},
                    {"text": " against account "},
                    {"key": "account_number"},
                    {"text": " is approved."},
                ],
            },
        ],
        # NEW: The Security Configuration
        "security": {
            "user_password": "Customer@123",  # The customer needs this to OPEN the PDF
            "owner_password": "BankAdmin@999",  # The bank needs this to print/copy/modify
        },
    }

    response = api_client.post(endpoint="/generate", json=payload)

    assert response.is_success, f"Failed to generate secure PDF: {response.text}"
    assert response.content is not None

    # Save the file
    save_file(f"{template_id}.pdf", response.content)

    # Optional: Verify the file is actually encrypted by checking the header
    # Encrypted PDFs contain the string "/Encrypt" in their raw bytes
    assert b"/Encrypt" in response.content, "PDF does not appear to be encrypted!"
