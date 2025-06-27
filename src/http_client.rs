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
use crate::error::{HttpError, HttpResult};
use crate::http_headers::HttpHeaders;
use crate::http_request::{HttpRequest, RetryPolicy};
use crate::http_response::HttpResponse;
use crate::http_url::HttpUrl;
use crate::interceptor::{PostRequestInterceptor, PreRequestInterceptor};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn execute(&self, request: HttpRequest) -> HttpResult<HttpResponse>;
}

pub struct ReqwestClient {
    client: reqwest::Client,
    pre_interceptors: Vec<Arc<dyn PreRequestInterceptor>>,
    post_interceptors: Vec<Arc<dyn PostRequestInterceptor>>,
    default_retry_policy: Option<RetryPolicy>,
    base_url: Option<HttpUrl>,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            pre_interceptors: Vec::new(),
            post_interceptors: Vec::new(),
            default_retry_policy: None,
            base_url: None,
        }
    }

    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = match HttpUrl::parse(base_url) {
            Ok(url) => Some(url),
            Err(e) => {
                panic!("Invalid base URL: {}", e);
            }
        };
        self
    }

    /// Set default headers for all requests. Overrides previous default headers if called multiple times.
    pub fn with_default_headers(mut self, headers: HttpHeaders) -> Self {
        let header_map = reqwest::header::HeaderMap::from(&headers);
        self.client = reqwest::Client::builder()
            .default_headers(header_map)
            .build()
            .expect("Failed to build reqwest client with default headers");
        self
    }

    pub fn with_default_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.default_retry_policy = Some(policy);
        self
    }

    pub fn with_pre_interceptor(mut self, interceptor: Arc<dyn PreRequestInterceptor>) -> Self {
        self.pre_interceptors.push(interceptor);
        self
    }

    pub fn with_post_interceptor(mut self, interceptor: Arc<dyn PostRequestInterceptor>) -> Self {
        self.post_interceptors.push(interceptor);
        self
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    async fn execute(&self, mut request: HttpRequest) -> HttpResult<HttpResponse> {
        // If base_url is set, join it with the request path (if relative) - do this FIRST
        if let Some(base_url) = &self.base_url {
            if let Some(ref path) = request.path {
                match base_url.join(path) {
                    Ok(full_url) => {
                        let mut_req = request.as_mut_reqwest();
                        *mut_req.url_mut() = full_url.0;
                    }
                    Err(e) => {
                        return Err(HttpError::Custom(format!(
                            "Failed to join base_url and path: {e}"
                        )));
                    }
                }
            }
        }

        // Pre-request interceptors (now they'll see the correct URL)
        for interceptor in &self.pre_interceptors {
            interceptor.intercept(&mut request).await;
        }

        // Append query parameters if present
        if let Some(ref params) = request.query_params {
            let mut url = request.as_reqwest().url().clone();
            {
                let mut pairs = url.query_pairs_mut();
                for (k, v) in params {
                    pairs.append_pair(k, v);
                }
            }
            let mut_req = request.as_mut_reqwest();
            *mut_req.url_mut() = url;
        }
        // Set JSON body if present
        if let Some(json) = &request.json_body {
            let json_string = json.to_string();
            let mut_req = request.as_mut_reqwest();
            *mut_req.body_mut() = Some(reqwest::Body::from(json_string));
            mut_req.headers_mut().insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
        }
        let retry_policy = request
            .retry_policy
            .clone()
            .or_else(|| self.default_retry_policy.clone());
        let mut attempts = 0;
        let max_attempts = retry_policy.as_ref().map_or(1, |p| p.max_attempts);
        let backoff = retry_policy.as_ref().map_or(0, |p| p.backoff_millis);
        loop {
            let reqwest_request = match request.as_reqwest().try_clone() {
                Some(r) => r,
                None => {
                    return Err(HttpError::Custom(
                        "Failed to clone request for retry".to_string(),
                    ));
                }
            };
            match self.client.execute(reqwest_request).await {
                Ok(response) => {
                    let mut http_response = HttpResponse::new(response);
                    for interceptor in &self.post_interceptors {
                        interceptor.intercept(&mut http_response).await;
                    }
                    return Ok(http_response);
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(HttpError::from(e));
                    }
                    if backoff > 0 {
                        sleep(Duration::from_millis(backoff)).await;
                    }
                }
            }
        }
    }
}
