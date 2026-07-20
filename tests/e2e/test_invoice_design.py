def test_invoice_design_layout(api_client, save_file):
    """Renders a blank/printable invoice template matching the reference
    design: solid diamond logo with an "S" monogram, brand header, orange
    accent rules, Invoice-to and Invoice#/Date blocks, a dark-header item
    table with blank rows, Payment Info + Sub Total/Tax/TOTAL block, terms,
    signature line, and an icon-enhanced footer contact bar.
    """
    ORANGE = "#F0955E"
    DARK = "#2B2B2E"
    GRAY = "#8A8A8E"

    payload = {
        "template_id": "invoice_design_v1",
        "format": "pdf",
        "data": {
            "invoice_number": "205092",
            "invoice_date": "12 / 12 / 2020",
            "client_name": "Mr. Jhon Doe",
            "client_address": "14 Dummy Street Area, Location,\nLorem Ipsum, 120xx15xx",
        },
        "content": [
            # ---------- Header: logo/brand (left) vs INVOICE (right) ----------
            {
                "type": "columns",
                "column_widths": ["1fr", "1fr"],
                "gutter": "1em",
                "items": [
                    [
                        {
                            "type": "columns",
                            "column_widths": ["auto", "1fr"],
                            "gutter": "0.5em",
                            "items": [
                                [
                                    # Solid diamond (logo mark) with a
                                    # centered white "S" overlaid on top —
                                    # matches the reference's brand mark,
                                    # not just an outline.
                                    {
                                        "type": "shape",
                                        "kind": "rect",
                                        "width": "0.9cm",
                                        "height": "0.9cm",
                                        "fill": ORANGE,
                                        "rotate": "45deg",
                                    },
                                    {
                                        "type": "placed",
                                        "anchor": "center",
                                        "content": {
                                            "type": "columns",
                                            "column_widths": ["0.9cm"],
                                            "items": [
                                                [
                                                    {
                                                        "type": "paragraph",
                                                        "alignment": "center",
                                                        "content": [
                                                            {
                                                                "text": "S",
                                                                "bold": True,
                                                                "color": "#ffffff",
                                                                "size": "16pt",
                                                            }
                                                        ],
                                                    }
                                                ]
                                            ],
                                        },
                                    },
                                ],
                                [
                                    {
                                        "type": "paragraph",
                                        "content": [
                                            {
                                                "text": "Brand Name",
                                                "bold": True,
                                                "size": "16pt",
                                            }
                                        ],
                                    },
                                    {
                                        "type": "paragraph",
                                        "content": [
                                            {
                                                "text": "TAGLINE SPACE HERE",
                                                "size": "7pt",
                                                "color": GRAY,
                                            }
                                        ],
                                    },
                                ],
                            ],
                        }
                    ],
                    [
                        {
                            "type": "heading",
                            "level": 1,
                            "alignment": "right",
                            "content": [{"text": "INVOICE"}],
                        }
                    ],
                ],
            },
            {"type": "spacer", "height": "0.3cm"},
            {
                "type": "shape",
                "kind": "rect",
                "width": "17cm",
                "height": "0.12cm",
                "fill": ORANGE,
            },
            {"type": "spacer", "height": "0.6cm"},
            # ---------- Invoice-to block vs Invoice #/Date block ----------
            {
                "type": "columns",
                "column_widths": ["1fr", "1fr"],
                "gutter": "1em",
                "items": [
                    [
                        {
                            "type": "paragraph",
                            "content": [{"text": "Invoice to:", "bold": True}],
                        },
                        {
                            "type": "paragraph",
                            "content": [{"key": "client_name"}],
                        },
                        {
                            "type": "paragraph",
                            "content": [{"key": "client_address"}],
                        },
                    ],
                    [
                        {
                            "type": "table",
                            "rows": [
                                [
                                    {"text": "Invoice #", "bold": True},
                                    {"key": "invoice_number"},
                                ],
                                [
                                    {"text": "Date", "bold": True},
                                    {"key": "invoice_date"},
                                ],
                            ],
                            "style": {
                                "columns": ["1fr", "1fr"],
                                "stroke": "none",
                                "column_align": ["left", "right"],
                                "inset": "4pt",
                            },
                        }
                    ],
                ],
            },
            {"type": "spacer", "height": "0.7cm"},
            # ---------- Item table (blank rows, ready to print/fill) ----------
            {
                "type": "table",
                "headers": ["No", "Item Description", "Qty", "Price", "Total"],
                "rows": [
                    [
                        {"text": ""},
                        {"text": ""},
                        {"text": ""},
                        {"text": ""},
                        {"text": ""},
                    ]
                    for _ in range(8)
                ],
                "style": {
                    "width": "100%",
                    "columns": ["0.6fr", "3fr", "1fr", "1.2fr", "1.2fr"],
                    "header_bg": DARK,
                    "column_align": ["center", "left", "center", "right", "right"],
                    "stroke": "0.5pt",
                    "inset": "8pt",
                },
            },
            {"type": "spacer", "height": "0.7cm"},
            # ---------- Payment info (left) vs Sub Total/Tax/TOTAL (right) ----------
            {
                "type": "columns",
                "column_widths": ["1.3fr", "1fr"],
                "gutter": "1.5em",
                "items": [
                    [
                        {
                            "type": "paragraph",
                            "content": [{"text": "Payment Info:", "bold": True}],
                        },
                        {
                            "type": "paragraph",
                            "content": [{"text": "Account #: 1234 5678 9012"}],
                        },
                        {
                            "type": "paragraph",
                            "content": [{"text": "A/C Name: Lorem Ipsum"}],
                        },
                        {
                            "type": "paragraph",
                            "content": [
                                {"text": "Bank Details: Add your bank details"}
                            ],
                        },
                    ],
                    [
                        {
                            "type": "table",
                            "rows": [
                                [
                                    {"text": "Sub Total", "bold": True, "fill": ORANGE},
                                    {"text": ""},
                                ],
                                [
                                    {"text": "Tax", "bold": True, "fill": ORANGE},
                                    {"text": ""},
                                ],
                                [
                                    {"text": "TOTAL", "bold": True, "fill": ORANGE},
                                    {"text": ""},
                                ],
                            ],
                            "style": {
                                "columns": ["1fr", "1fr"],
                                "stroke": "0.5pt",
                                "column_align": ["left", "right"],
                                "inset": "6pt",
                            },
                        }
                    ],
                ],
            },
            {"type": "spacer", "height": "0.8cm"},
            # ---------- Terms & Conditions ----------
            {
                "type": "paragraph",
                "content": [{"text": "Term & Condition", "bold": True}],
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "text": "Lorem Ipsum dolor sit amet, consectetur adipiscing "
                        "Elit. Fusce dignissim pretium consectetur.",
                        "color": GRAY,
                    }
                ],
            },
            {"type": "spacer", "height": "0.6cm"},
            # ---------- Thank you + signature line ----------
            {
                "type": "columns",
                "column_widths": ["1fr", "1fr"],
                "gutter": "1em",
                "items": [
                    [
                        {
                            "type": "paragraph",
                            "content": [{"text": "Thanks for your business."}],
                        }
                    ],
                    [
                        {
                            "type": "shape",
                            "kind": "rect",
                            "width": "5cm",
                            "height": "0.03cm",
                            "fill": "#000000",
                        },
                        {
                            "type": "paragraph",
                            "alignment": "right",
                            "content": [{"text": "Authorised Sign"}],
                        },
                    ],
                ],
            },
            {"type": "spacer", "height": "1cm"},
            # ---------- Footer contact bar ----------
            {
                "type": "shape",
                "kind": "rect",
                "width": "17cm",
                "height": "0.08cm",
                "fill": ORANGE,
            },
            {"type": "spacer", "height": "0.3cm"},
            # Icons: "☎" and "✉" are real glyphs (verified present in a
            # commonly-available fallback font); the pin and globe icons
            # use plain geometric shapes instead of emoji, since color-emoji
            # codepoints (📍🌐) are NOT covered by that font and would risk
            # rendering as blank "tofu" boxes depending on what's installed
            # on the server. This trades a little icon fidelity for a
            # guarantee that nothing renders as a missing-glyph box.
            {
                "type": "columns",
                "column_widths": [
                    "1fr",
                    "auto",
                    "1fr",
                    "auto",
                    "1fr",
                    "auto",
                    "1fr",
                ],
                "gutter": "0.5em",
                "items": [
                    [
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [
                                {"text": "☎ ", "size": "8pt", "color": GRAY},
                                {"text": "Phone", "size": "8pt", "color": GRAY},
                            ],
                        }
                    ],
                    [
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [{"text": "|", "color": GRAY}],
                        }
                    ],
                    [
                        {
                            "type": "shape",
                            "kind": "circle",
                            "width": "0.2cm",
                            "height": "0.2cm",
                            "fill": GRAY,
                        },
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [
                                {"text": "Address", "size": "8pt", "color": GRAY}
                            ],
                        },
                    ],
                    [
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [{"text": "|", "color": GRAY}],
                        }
                    ],
                    [
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [
                                {"text": "✉ ", "size": "8pt", "color": GRAY},
                                {"text": "Mail", "size": "8pt", "color": GRAY},
                            ],
                        }
                    ],
                    [
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [{"text": "|", "color": GRAY}],
                        }
                    ],
                    [
                        {
                            "type": "shape",
                            "kind": "circle",
                            "width": "0.22cm",
                            "height": "0.22cm",
                            "fill": GRAY,
                        },
                        {
                            "type": "paragraph",
                            "alignment": "center",
                            "content": [
                                {"text": "Website", "size": "8pt", "color": GRAY}
                            ],
                        },
                    ],
                ],
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    save_file("invoice_design_v1.pdf", response.content)
