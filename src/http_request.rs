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
use crate::http_method::HttpMethod;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_millis: u64,
}

#[derive(Debug)]
pub struct HttpRequest {
    inner: reqwest::Request,
    pub retry_policy: Option<RetryPolicy>,
    pub path: Option<String>,
    pub query_params: Option<HashMap<String, String>>,
    pub json_body: Option<Value>,
}

impl HttpRequest {
    // Builder methods for creating requests without reqwest
    pub fn new(method: HttpMethod, path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a placeholder URL that will be replaced by the client with the base URL
        let placeholder_url = reqwest::Url::parse("http://placeholder.com")?;
        let reqwest_method: reqwest::Method = method.into();
        let inner = reqwest::Request::new(reqwest_method, placeholder_url);
        Ok(Self {
            inner,
            retry_policy: None,
            path: Some(path.to_string()),
            query_params: None,
            json_body: None,
        })
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_query_params(mut self, params: HashMap<String, String>) -> Self {
        self.query_params = Some(params);
        self
    }

    pub fn with_json_body(mut self, body: Value) -> Self {
        self.json_body = Some(body);
        self
    }

    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    // Header manipulation methods
    pub fn add_header(
        &mut self,
        name: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header_name = reqwest::header::HeaderName::from_bytes(name.as_bytes())?;
        let header_value = reqwest::header::HeaderValue::from_str(value)?;
        self.inner.headers_mut().insert(header_name, header_value);
        Ok(())
    }

    pub fn get_method(&self) -> &str {
        self.inner.method().as_str()
    }

    pub fn get_url_path(&self) -> &str {
        self.inner.url().path()
    }

    // Accessor methods
    pub fn as_reqwest(&self) -> &reqwest::Request {
        &self.inner
    }
    pub fn as_mut_reqwest(&mut self) -> &mut reqwest::Request {
        &mut self.inner
    }
    pub fn into_inner(self) -> reqwest::Request {
        self.inner
    }
    pub fn set_url(&mut self, url: reqwest::Url) {
        *self.inner.url_mut() = url;
    }
}
