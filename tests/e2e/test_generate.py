# tests/e23/test_generate.py

import pytest

from tests.utils import get_logo_uri


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
            "details": {
                "customer_name": "Amrit Giri",
                "loan_amount": "NPR 5,00,000",
                "interest_rate": "12.5%",
                "tenure": "5 Years",
                "today_date": "July 09, 2026",
                "collaterals": [
                    {
                        "type": "Residential Land",
                        "location": "Kathmandu-32",
                        "value": "3,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Gold Jewelry",
                        "location": "Bank Vault",
                        "value": "2,00,000",
                    },
                    {
                        "type": "Fixed Deposit",
                        "location": "Siddhartha Bank",
                        "value": "1,50,000",
                    },
                    {
                        "type": "Vehicle (Car)",
                        "location": "Garage",
                        "value": "4,50,000",
                    },
                ],
                # "total_valuation": "=sum(value)",
            },
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
                    "dy": "2cm",
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
            # Loan Sanction Letter
            {"type": "page_break"},
            {
                "type": "heading",
                "level": 3,
                "content": [{"text": "LOAN SANCTION LETTER"}],
            },
            {
                "type": "paragraph",
                "content": [
                    {"text": "Date: "},
                    {"key": "details.today_date"},  # This is now a VariableNode
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {"text": "Dear "},
                    {"key": "details.customer_name"},
                    {"text": ","},
                ],
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "We are pleased to inform you that your loan application has been approved."
                    }
                ],
            },
            {
                "type": "table",
                "headers": ["Description", "Details"],
                "rows": [
                    [{"key": "details.loan_amount"}, {"key": "details.interest_rate"}],
                    [{"key": "details.tenure"}, {"key": "details.customer_name"}],
                ],
                "style": {
                    "width": "100%",
                    "header_bg": "#f3f4f6",  # Light gray background for header
                    "stroke": "0.5pt",
                    "inset": "10pt",  # More padding
                },
            },
            {
                "type": "table",
                "headers": ["Description", "Amount"],
                "rows": [
                    [{"key": "details.customer_name"}, {"key": "details.loan_amount"}],
                ],
                "style": {
                    "width": "100%",
                    "columns": [
                        "2fr",
                        "1fr",
                    ],  # First column is 2x wider than the second
                    "header_bg": "#1f2937",  # Dark gray header
                    "stroke": "none",  # No borders!
                },
            },
            {
                "type": "table",
                "headers": ["Item", "Price"],
                "rows": [
                    [{"key": "details.customer_name"}, {"key": "details.loan_amount"}]
                ],
                "style": {
                    "width": "10cm",  # Fixed width table
                    "columns": ["auto", "auto"],  # Auto-size columns to fit content
                    "inset": "5pt",
                },
            },
            # new
            {
                "type": "heading",
                "level": 2,
                "alignment": "center",
                "content": [{"text": "Collateral Valuation Schedule"}],
            },
            {
                "type": "paragraph",
                "content": [
                    {"text": "Loan Ref: "},
                    {"key": "details.loan_id"},
                    {"text": " | Customer: "},
                    {"key": "details.customer_name"},
                ],
            },
            {
                "type": "table",
                "headers": [
                    "S.N.",
                    "Asset Type",
                    "Location / Custodian",
                    "Valuation (NPR)",
                ],
                # DYNAMIC LOOP BODY
                "loop_data": "details.collaterals",
                "row_template": [
                    {"key": "__index"},  # Magic variable for Auto S.N. (1, 2, 3...)
                    {"key": "type"},  # Resolved from local 'item'
                    {"key": "location"},  # Resolved from local 'item'
                    {"key": "value"},  # Resolved from local 'item'
                ],
                # FOOTER (TOTALS)
                "footer": [
                    {"text": "", "colspan": 2, "rowspan": 2, "border": None},
                    {
                        "text": "Total Valuation",
                        "bold": True,
                    },  # Note: requires bold support in VariableCell or we just use text
                    {"formula": "=sum(value)", "locale": "en-Np", "bold": True},
                    {
                        "text": "Total",
                        "bold": True,
                    },  # Note: requires bold support in VariableCell or we just use text
                    {"formula": "=sum(value)", "locale": "en-Np", "bold": True},
                ],
                "style": {
                    "columns": ["1fr", "3fr", "3fr", "2fr"],  # Proportional widths
                    "header_bg": "#1f2937",
                    "stroke": "0.5pt",
                    "inset": "8pt",
                },
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "Please visit our branch to complete the final documentation."
                    }
                ],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
