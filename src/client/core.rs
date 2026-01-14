use crate::error::{ApiError, ApiResult};
use crate::models::ErrorResponse;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, Response, StatusCode, header::HeaderMap, header::HeaderValue};
use std::collections::HashMap;

use super::SureClient;

impl SureClient {
    /// Core request execution logic
    pub(crate) async fn execute_request<T>(
        &self,
        method: Method,
        path: &str,
        query_params: Option<&HashMap<&str, String>>,
        body: Option<String>,
    ) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // 1. Build URL
        let url = if let Some(params) = query_params {
            reqwest::Url::parse_with_params(&format!("{}{}", self.base_url, path), params)
                .map_err(ApiError::UrlParse)?
        } else {
            reqwest::Url::parse(&format!("{}{}", self.base_url, path))
                .map_err(ApiError::UrlParse)?
        };

        // 2. Build headers
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        // Set authentication header based on auth type
        match &self.auth {
            crate::types::Auth::Bearer(token) => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.as_str()))
                        .map_err(ApiError::InvalidHeaderValue)?,
                );
            }
            crate::types::Auth::ApiKey(key) => {
                headers.insert(
                    "X-Api-Key",
                    HeaderValue::from_str(key.as_str()).map_err(ApiError::InvalidHeaderValue)?,
                );
            }
        }

        if body.is_some() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        // 3. Build and execute request
        let mut request_builder = self.client.request(method, url).headers(headers);

        if let Some(body_str) = body {
            request_builder = request_builder.body(body_str);
        }

        let response = request_builder.send().await.map_err(ApiError::Network)?;

        // 4. Handle response
        if response.status().is_success() {
            self.handle_success_response(response).await
        } else {
            self.handle_error_response(response).await
        }
    }

    /// Handle successful responses
    async fn handle_success_response<T>(&self, res: Response) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = res.text().await.map_err(ApiError::Network)?;
        serde_json::from_str(&text).map_err(|error| ApiError::JsonDeserialization {
            error,
            source_string: text,
        })
    }

    /// Handle error responses
    async fn handle_error_response<T>(&self, res: Response) -> ApiResult<T> {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| status.to_string());

        // Try parsing as structured error response
        let message = if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&text) {
            error_response
                .message
                .or(Some(error_response.error))
                .unwrap_or_else(|| text.clone())
        } else {
            // Try parsing as JSON with "message" or "error" field
            serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| {
                    v.get("message")
                        .and_then(|m| m.as_str())
                        .or_else(|| v.get("error").and_then(|e| e.as_str()))
                        .map(String::from)
                })
                .unwrap_or(text)
        };

        // Map to specific error variants
        Err(match status {
            StatusCode::BAD_REQUEST => ApiError::BadRequest { message, status },
            StatusCode::UNAUTHORIZED => ApiError::Unauthorized { message },
            StatusCode::FORBIDDEN => ApiError::Forbidden { message },
            StatusCode::NOT_FOUND => ApiError::NotFound { message },
            StatusCode::UNPROCESSABLE_ENTITY => ApiError::ValidationError { message },
            StatusCode::TOO_MANY_REQUESTS => ApiError::RateLimited { message },
            StatusCode::INTERNAL_SERVER_ERROR => ApiError::InternalServerError { message },
            _ => ApiError::ApiError { status, message },
        })
    }
}
