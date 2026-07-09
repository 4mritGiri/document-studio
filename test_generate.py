import json

import requests

url = "http://localhost:3000/generate"

payload = {
    "template_id": "mlms_proposal_v1",
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
                {"type": "Residential Land", "value": "NPR 3,00,000"},
                {"type": "Gold Jewelry", "value": "NPR 2,00,000"},
                {"type": "Gold Jewelry", "value": "NPR 2,00,000"},
                {"type": "Gold Jewelry", "value": "NPR 2,00,000"},
                {"type": "Gold Jewelry", "value": "NPR 2,00,000"},
                {"type": "Gold Jewelry", "value": "NPR 2,00,000"},
            ],
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
                    {"text": " - Ensuring proactive management of credit exposure."},
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
        {"type": "paragraph", "alignment": "right", "content": [{"key": "sign_title"}]},
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
        {"type": "heading", "level": 3, "content": [{"text": "LOAN SANCTION LETTER"}]},
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
                "columns": ["2fr", "1fr"],  # First column is 2x wider than the second
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
        {
            "type": "paragraph",
            "content": [
                {"text": "Please visit our branch to complete the final documentation."}
            ],
        },
    ],
}

response = requests.post(url, json=payload)

if response.status_code == 200:
    with open("mlms_proposal.pdf", "wb") as f:
        f.write(response.content)
    print("✅ Success! 'mlms_proposal.pdf' has been generated.")
else:
    print(f"❌ Error: {response.status_code}")
    print(response.text)
