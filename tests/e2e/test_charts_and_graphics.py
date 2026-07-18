# tests/e2e/test_charts_and_graphics.py
import pytest


@pytest.mark.parametrize("output_format", ["html", "pdf"])
def test_charts_and_svg_paths(api_client, save_file, output_format):
    """Tests native chart generation and advanced SVG paths."""
    template_id = "chart_and_svg_paths"
    payload = {
        "template_id": template_id,
        "format": output_format,
        "data": {"title": "Q3 Financial Performance"},
        "content": [
            {"type": "heading", "level": 1, "content": [{"key": "title"}]},
            # 1. BAR CHART
            {
                "type": "heading",
                "level": 2,
                "content": [{"text": "Revenue by Branch (Bar)"}],
            },
            {
                "type": "chart",
                "chart_type": "bar",
                "title": "Branch Revenue (NPR in Lakhs)",
                "data": [
                    {"label": "Kathmandu", "value": 150},
                    {"label": "Pokhara", "value": 85},
                    {"label": "Chitwan", "value": 110},
                    {"label": "Biratnagar", "value": 60},
                ],
                "width": "12cm",
                "height": "7cm",
            },
            # 2. LINE CHART
            {
                "type": "heading",
                "level": 2,
                "content": [{"text": "Interest Rate Trend (Line)"}],
            },
            {
                "type": "chart",
                "chart_type": "line",
                "title": "Base Rate Over 6 Months",
                "data": [
                    {"label": "Jan", "value": 8.5},
                    {"label": "Feb", "value": 8.2},
                    {"label": "Mar", "value": 8.0},
                    {"label": "Apr", "value": 7.8},
                    {"label": "May", "value": 7.5},
                    {"label": "Jun", "value": 7.2},
                ],
                "width": "12cm",
                "height": "7cm",
            },
            # 3. ADVANCED SVG PATH (Custom Brand Logo/Shape)
            {
                "type": "heading",
                "level": 2,
                "content": [{"text": "Custom SVG Path (Brand Asset)"}],
            },
            {
                "type": "shape",
                "kind": "path",
                # A custom geometric star/arrow path
                "path_data": "M50,10 L60,40 L90,40 L65,60 L75,90 L50,70 L25,90 L35,60 L10,40 L40,40 Z",
                "width": "4cm",
                "height": "4cm",
                "fill": "#f59e0b",  # Amber
                "stroke": "#b45309",  # Dark Amber
                "stroke_width": "2",
            },
        ],
    }

    response = api_client.post(endpoint="/generate", json=payload)
    assert response.is_success, f"Failed: {response.text}"
    assert response.content is not None

    save_file(f"{template_id}.{output_format}", response.content)
