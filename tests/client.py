# tests/client.py
import os
from typing import Any, Dict, Optional

import httpx


class ApiResponse:
    def __init__(self, response: httpx.Response):
        self.response = response
        self.status_code = response.status_code
        self.is_success = 200 <= self.status_code < 300
        self.headers = response.headers
        self.content: Optional[bytes] = None
        self.json: Optional[Dict[str, Any]] = None
        self.text: Optional[str] = None

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

    def _make_request(self, method: str, endpoint: str, **kwargs) -> ApiResponse:
        with httpx.Client(
            base_url=self.base_url, timeout=self.timeout, headers=self.headers
        ) as client:
            response = client.request(method, endpoint, **kwargs)
            if os.getenv("DEBUG_API"):
                print(f"\nDEBUG: {method.upper()} {endpoint} -> {response.status_code}")
                print(f"DEBUG: Content-Type: {response.headers.get('Content-Type')}")
            response.raise_for_status()
            return ApiResponse(response)

    def post(self, endpoint: str, json: Dict[str, Any]) -> ApiResponse:
        return self._make_request("POST", endpoint, json=json)

    def get(
        self, endpoint: str, params: Optional[Dict[str, Any]] = None
    ) -> ApiResponse:
        return self._make_request("GET", endpoint, params=params)
