/**
 * Copyright 2025-present Coinbase Global, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::http_status_code::HttpStatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    inner: reqwest::Response,
}

impl HttpResponse {
    pub fn new(inner: reqwest::Response) -> Self {
        Self { inner }
    }
    pub fn as_reqwest(&self) -> &reqwest::Response {
        &self.inner
    }
    pub fn as_mut_reqwest(&mut self) -> &mut reqwest::Response {
        &mut self.inner
    }
    pub fn into_inner(self) -> reqwest::Response {
        self.inner
    }

    /// Get the HTTP status code from the response
    pub fn status(&self) -> HttpStatusCode {
        self.inner.status().into()
    }

    pub async fn json<T: serde::de::DeserializeOwned>(self) -> crate::error::HttpResult<T> {
        let status = self.inner.status();
        let url = self.inner.url().clone();

        // Get the response body as bytes first
        let bytes = match self.inner.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => return Err(crate::error::HttpError::Reqwest(e)),
        };

        // Try to deserialize from the bytes
        match serde_json::from_slice::<T>(&bytes) {
            Ok(data) => Ok(data),
            Err(serde_err) => {
                // If deserialization failed, try to get the response body as text for better error reporting
                let body_text = String::from_utf8_lossy(&bytes);

                // Create a more detailed error message
                let error_msg = format!(
                    "Failed to deserialize JSON response from {} (status: {}). \
                    Response body: {}. \
                    Serde error: {}",
                    url, status, body_text, serde_err
                );

                Err(crate::error::HttpError::Custom(error_msg))
            }
        }
    }

    pub async fn json_bytes(self) -> crate::error::HttpResult<bytes::Bytes> {
        self.inner
            .bytes()
            .await
            .map_err(crate::error::HttpError::from)
    }

    pub async fn text(self) -> crate::error::HttpResult<String> {
        self.inner
            .text()
            .await
            .map_err(crate::error::HttpError::from)
    }
}
