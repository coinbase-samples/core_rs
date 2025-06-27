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
use reqwest::Url;
use std::fmt;

#[derive(Debug, Clone)]
pub struct HttpUrl(pub Url);

#[derive(Debug)]
pub struct HttpUrlError(pub String);

impl fmt::Display for HttpUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpUrl error: {}", self.0)
    }
}

impl std::error::Error for HttpUrlError {}

impl HttpUrl {
    pub fn parse(url: &str) -> Result<Self, HttpUrlError> {
        Url::parse(url)
            .map(HttpUrl)
            .map_err(|e| HttpUrlError(e.to_string()))
    }
    pub fn join(&self, path: &str) -> Result<Self, HttpUrlError> {
        self.0
            .join(path)
            .map(HttpUrl)
            .map_err(|e| HttpUrlError(e.to_string()))
    }
}
