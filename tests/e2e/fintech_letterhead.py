# tests/e23/test_generate.py

import pytest

from tests.utils import get_logo_uri

template_id = "fintech_letterhead"


@pytest.mark.parametrize("output_format", ["pdf", "html"])
def test_mlms_invoice(api_client, save_file, output_format):
    """Tests MLMS invoice generation."""
    template_id = "mlms_invoice"

    payload = {
        "template_id": template_id,
        "format": output_format,
        "data": {
            "date": "4th August 2025",
            "company_name": "Siddhartha Bank Ltd.",
            "address": "Naxal, Kathmandu",
            "sign_name": "Hemanta Ghimire",
            "sign_title": "Co-Founder",
            "sign_company": "Fintech Nepal Pvt Ltd.",
            "sign_address": "Kathmandu-32, Nepal",
            "sign_phone": "+977-9851110953",
            "sign_email": "rabi.maharjan@fintechnepal.com",
        },
        "page": {
            # This is the reusable "letterhead" — it's drawn behind every page,
            # the same way a Word letterhead template works.
            "background": [
                # Solid diagonal cut in the topleft to center corner
                {
                    "type": "placed",
                    "anchor": "top-left",
                    # "dx": "1cm",
                    # "dy": "0.1cm",
                    "content": {
                        "type": "shape",
                        "kind": "triangle",
                        "width": "18cm",
                        "height": "0.88cm",
                        # "rotate": "359deg",
                        "fill": "#323139",
                    },
                },
                # Solid diagonal cut in the top-left corner
                {
                    "type": "placed",
                    "anchor": "top-left",
                    "dx": "-2.2cm",
                    "dy": "-0.91cm",
                    "content": {
                        "type": "shape",
                        "kind": "triangle",
                        "width": "3.1cm",
                        "height": "3cm",
                        "rotate": "142deg",
                        "fill": "#34364b",
                    },
                },
                # Solid rectangle cut in the topleft to center corner
                {
                    "type": "placed",
                    "anchor": "top-left",
                    "dx": "-0.8cm",
                    # "dy": "-0.165cm",
                    "content": {
                        "type": "shape",
                        "kind": "rect",
                        "width": "3cm",
                        "height": "0.2cm",
                        "rotate": "45deg",
                        "fill": "#ffffff",
                    },
                },
                # Logo pinned top-left, on top of the letterhead
                {
                    "type": "placed",
                    "anchor": "top-left",
                    "dx": "2cm",
                    "dy": "1cm",
                    "content": {
                        "type": "image",
                        "src": get_logo_uri(),
                        "width": "2cm",
                    },
                },
                {
                    "type": "placed",
                    "anchor": "top-left",
                    "dx": "4.5cm",
                    "dy": "1cm",
                    "content": {
                        "type": "heading",
                        "level": 1,
                        "content": [{"text": "FINTECH", "bold": True}],
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
                        "fill": "#323139",
                        "rotate": "180deg",
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
            "watermark": {
                "text": "DRAFT",  # Or "CONFIDENTIAL", "COPY"
                "opacity": 0.15,  # Very subtle, won't obscure the text
                # "angle": 120.0,  # Diagonal from bottom-left to top-right
                "font_size": "60pt",  # Large enough to span the page
                # "color": "#ff0000",  # Red for high visibility
            },
        },
        "content": [
            # 1. Right-aligned Date
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"text": "Date: "}, {"key": "date"}],
            },
            # 2. Recipient Address (using \n for line breaks)
            {
                "type": "paragraph",
                "content": [
                    {"text": "To.\nThe CEO,\n"},
                    {"key": "company_name"},
                    {"text": "\n"},
                    {"key": "address"},
                ],
            },
            # 3. Subject Line (Bold)
            {
                "type": "paragraph",
                "alignment": "center",
                "content": [
                    {"text": "Subject: ", "bold": True},
                    {"text": "Proposal for Margin Lending Management System"},
                ],
            },
            # 4. Salutation
            {"type": "paragraph", "content": [{"text": "Dear Sir,"}]},
            # 5. Body Paragraphs
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "I am writing to introduce our innovative Margin Lending Management System (MLMS), designed to enhance operational efficiency, automate processes, and ensure strict compliance with credit management regulations."
                    }
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "Fintech Nepal specializes in delivering cutting-edge financial technology solutions. Our proposed system is tailored to address key challenges in margin lending, including:"
                    }
                ],
            },
            # 6. Bullet Points
            {
                "type": "bullet_list",
                "items": [
                    [
                        {"text": "Automated Margin Call Processing", "bold": True},
                        {
                            "text": " - Reducing manual intervention and improving response times."
                        },
                    ],
                    [
                        {"text": "Real-Time Risk Monitoring", "bold": True},
                        {
                            "text": " - Ensuring proactive management of credit exposure."
                        },
                    ],
                    [
                        {"text": "Regulatory Compliance Framework", "bold": True},
                        {"text": " - Aligning with Central Bank guidelines."},
                    ],
                    [
                        {"text": "Seamless Integration", "bold": True},
                        {
                            "text": " - Compatible with existing core banking systems for minimal disruption."
                        },
                    ],
                ],
            },
            # 7. Closing Paragraphs
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "By implementing our solution, Siddhartha Bank can achieve enhanced operational efficiency, reduced risk exposure, and full audit trails for compliance reporting."
                    }
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "We would appreciate the opportunity to present a detailed proposal. Please let us know a convenient time for a meeting at your earliest availability."
                    }
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "Thank you for considering our proposal. I look forward to your positive response."
                    }
                ],
            },
            # 8. Sign-off
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"text": "Best Regards,"}],
            },
            {
                "type": "spacer",
                "height": "1cm",  # <--- Adjust this value (e.g., "0.5cm", "12pt")
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"text": "Hemanta Ghimire", "bold": True}],
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"key": "sign_title"}],
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"key": "sign_company"}],
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [{"key": "sign_address"}],
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [
                    {"text": "Contact No: "},
                    {"key": "sign_phone"},
                ],
            },
            {
                "type": "paragraph",
                "alignment": "right",
                "content": [
                    {"text": "Email: "},
                    {"key": "sign_email"},
                ],
            },
        ],
    }
    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
