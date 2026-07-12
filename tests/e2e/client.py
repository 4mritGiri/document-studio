import os
from typing import Any, Dict, Optional

import httpx


class ApiResponse:
    def __init__(self, response: httpx.Response):
        self.response = response
        self.status_code = response.status_code
        self.is_success = 200 <= self.status_code < 300
        self.content: Optional[bytes] = None
        self.json: Optional[Dict[str, Any]] = None
        self.text: Optional[str] = None
        self.headers: Dict[str, str] = response.headers

        content_type = response.headers.get("Content-Type", "")
        if "application/json" in content_type:
            self.json = response.json()
        else:
            # Explicitly cast or assign the raw bytes
            self.content = response.content
            self.text = response.text

    def __repr__(self):
        return f"<ApiResponse status_code={self.status_code} has_json={self.json is not None}>"


class APIClient:
    def __init__(
        self, base_url: str, timeout: float = 10.0, api_key: Optional[str] = None
    ):
        self.base_url = base_url
        self.timeout = timeout
        self.headers = {"Content-Type": "application/json"}

        if api_key:
            self.headers["x-api-key"] = f"{api_key}"

    def get(
        self, endpoint: str, params: Optional[Dict[str, Any]] = None
    ) -> ApiResponse:
        """Performs a GET request."""
        with httpx.Client(
            base_url=self.base_url, timeout=self.timeout, headers=self.headers
        ) as client:
            response = client.get(endpoint, params=params)
            return ApiResponse(response)

    def post(self, endpoint: str, json: Dict[str, Any]) -> ApiResponse:
        with httpx.Client(
            base_url=self.base_url, timeout=self.timeout, headers=self.headers
        ) as client:
            response = client.post(endpoint, json=json)
            # DEBUG: See what the server is actually sending
            print(f"DEBUG: Content-Type: {response.headers.get('Content-Type')}")
            print(f"DEBUG: First 20 bytes: {response.content[:20]}")

            response.raise_for_status()
            return ApiResponse(response)


# Example Usage:
apiKey = os.environ.get(
    "DOCUMENT_ENGINE_API_KEY",
    "90de5ac5c9501019b9d3ae98f7503d82a5e1de1a7c49213fc3559c91f7b01a56",
)
requests = APIClient(base_url="http://localhost:3000/api/v1/", api_key=apiKey)
