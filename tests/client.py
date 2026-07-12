# tests/client.py
from typing import Any, Dict, Optional

import httpx


class ApiResponse:
    def __init__(self, response: httpx.Response):
        self.response = response
        self.status_code = response.status_code
        self.is_success = 200 <= self.status_code < 300
        self.headers = response.headers

        content_type = response.headers.get("Content-Type", "")
        if "application/json" in content_type:
            try:
                self.json = response.json()
                self.text = response.text
            except Exception:
                self.content = response.content
                self.text = response.text
        else:
            self.content = response.content
            self.text = response.text

    def __repr__(self):
        return f"<ApiResponse status_code={self.status_code} has_json={self.json is not None}>"


class APIClient:
    def __init__(
        self, base_url: str, timeout: float = 30.0, api_key: Optional[str] = None
    ):
        self.base_url = base_url
        self.timeout = timeout
        self.headers = {"Content-Type": "application/json"}
        if api_key:
            self.headers["x-api-key"] = api_key

    def post(self, endpoint: str, json: Dict[str, Any]) -> ApiResponse:
        with httpx.Client(
            base_url=self.base_url, timeout=self.timeout, headers=self.headers
        ) as client:
            response = client.post(endpoint, json=json)
            return ApiResponse(response)

    def get(
        self, endpoint: str, params: Optional[Dict[str, Any]] = None
    ) -> ApiResponse:
        with httpx.Client(
            base_url=self.base_url, timeout=self.timeout, headers=self.headers
        ) as client:
            response = client.get(endpoint, params=params)
            return ApiResponse(response)
