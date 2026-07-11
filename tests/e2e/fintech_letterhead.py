# tests/e23/test_generate.py

import base64
import mimetypes
import os

import requests


def to_data_uri(path: str) -> str:
    mime_type, _ = mimetypes.guess_type(path)
    mime_type = mime_type or "image/png"
    with open(path, "rb") as f:
        encoded = base64.b64encode(f.read()).decode("ascii")
    return f"data:{mime_type};base64,{encoded}"


url = "http://localhost:3000/api/v1/generate"

LOGO_PATH = "/media/amrit/SSDAmrit/Builds/local/DocumentStudio/assets/images/logo.png"
OUTPUT_DIR = "docs/examples/"
PLACEHOLDER_LOGO = to_data_uri(LOGO_PATH)
template_id = "fintech_letterhead"
format = "html"
# format = "pdf"

payload = {
    "template_id": template_id,
    "format": format,
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
                    "src": PLACEHOLDER_LOGO,
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
    ],
}

api_key = os.environ.get(
    "DOCUMENT_ENGINE_API_KEY",
    "90de5ac5c9501019b9d3ae98f7503d82a5e1de1a7c49213fc3559c91f7b01a56",
)
headers = {"x-api-key": f"{api_key}"}
response = requests.post(url, json=payload, headers=headers)

if response.status_code == 200:
    # with open(OUTPUT_DIR + "mlms_proposal.html", "wb") as f:
    with open(OUTPUT_DIR + f"{template_id}.{format}", "wb") as f:
        f.write(response.content)
    print(f"✅ Success! '{OUTPUT_DIR}{template_id}.{format}' has been generated.")
else:
    print(f"❌ Error: {response.status_code}")
    print(response.text)
