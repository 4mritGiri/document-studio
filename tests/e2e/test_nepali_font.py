# tests/e2e/test_nepali_font.py
from client import requests

format = "pdf"
template_id = "nepali_test"

payload = {
    "template_id": template_id,
    "format": format,
    "data": {"name_np": "अमृत गिरी", "address_np": "काठमाडौं, नेपाल"},
    "page": {
        # Set the DEFAULT font for the whole document to the Nepali font
        "default_font": "Noto Sans Devanagari"
    },
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
                # Example of INLINE font switching (back to English for one word)
                {"text": "Customer Name: ", "font_family": "Times New Roman"},
                {"key": "name_np"},
            ],
        },
    ],
}


response = requests.post(endpoint="/generate", data=payload)

if response.status_code == 200 and response.content is not None:
    with open(f"{template_id}.{format}", "wb") as f:
        f.write(response.content)
    print(f"✅ Success! Check '{template_id}.{format}'")
else:
    print(f"❌ Error: {response.response.text}")
