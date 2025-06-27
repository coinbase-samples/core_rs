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
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct HttpHeaders(pub HashMap<String, String>);

impl HttpHeaders {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }
}

impl From<&HttpHeaders> for reqwest::header::HeaderMap {
    fn from(headers: &HttpHeaders) -> Self {
        let mut map = reqwest::header::HeaderMap::new();
        for (k, v) in &headers.0 {
            if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(k.as_bytes()) {
                if let Ok(header_value) = reqwest::header::HeaderValue::from_str(v) {
                    map.insert(header_name, header_value);
                }
            }
        }
        map
    }
}
