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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum HttpStatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,

    Custom(u16),
}

impl HttpStatusCode {
    /// Get the numeric status code
    pub fn as_u16(&self) -> u16 {
        match self {
            HttpStatusCode::Continue => 100,
            HttpStatusCode::SwitchingProtocols => 101,
            HttpStatusCode::Processing => 102,
            HttpStatusCode::EarlyHints => 103,
            HttpStatusCode::Ok => 200,
            HttpStatusCode::Created => 201,
            HttpStatusCode::Accepted => 202,
            HttpStatusCode::NonAuthoritativeInformation => 203,
            HttpStatusCode::NoContent => 204,
            HttpStatusCode::ResetContent => 205,
            HttpStatusCode::PartialContent => 206,
            HttpStatusCode::MultiStatus => 207,
            HttpStatusCode::AlreadyReported => 208,
            HttpStatusCode::ImUsed => 226,
            HttpStatusCode::MultipleChoices => 300,
            HttpStatusCode::MovedPermanently => 301,
            HttpStatusCode::Found => 302,
            HttpStatusCode::SeeOther => 303,
            HttpStatusCode::NotModified => 304,
            HttpStatusCode::UseProxy => 305,
            HttpStatusCode::TemporaryRedirect => 307,
            HttpStatusCode::PermanentRedirect => 308,
            HttpStatusCode::BadRequest => 400,
            HttpStatusCode::Unauthorized => 401,
            HttpStatusCode::PaymentRequired => 402,
            HttpStatusCode::Forbidden => 403,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::MethodNotAllowed => 405,
            HttpStatusCode::NotAcceptable => 406,
            HttpStatusCode::ProxyAuthenticationRequired => 407,
            HttpStatusCode::RequestTimeout => 408,
            HttpStatusCode::Conflict => 409,
            HttpStatusCode::Gone => 410,
            HttpStatusCode::LengthRequired => 411,
            HttpStatusCode::PreconditionFailed => 412,
            HttpStatusCode::PayloadTooLarge => 413,
            HttpStatusCode::UriTooLong => 414,
            HttpStatusCode::UnsupportedMediaType => 415,
            HttpStatusCode::RangeNotSatisfiable => 416,
            HttpStatusCode::ExpectationFailed => 417,
            HttpStatusCode::ImATeapot => 418,
            HttpStatusCode::MisdirectedRequest => 421,
            HttpStatusCode::UnprocessableEntity => 422,
            HttpStatusCode::Locked => 423,
            HttpStatusCode::FailedDependency => 424,
            HttpStatusCode::TooEarly => 425,
            HttpStatusCode::UpgradeRequired => 426,
            HttpStatusCode::PreconditionRequired => 428,
            HttpStatusCode::TooManyRequests => 429,
            HttpStatusCode::RequestHeaderFieldsTooLarge => 431,
            HttpStatusCode::UnavailableForLegalReasons => 451,
            HttpStatusCode::InternalServerError => 500,
            HttpStatusCode::NotImplemented => 501,
            HttpStatusCode::BadGateway => 502,
            HttpStatusCode::ServiceUnavailable => 503,
            HttpStatusCode::GatewayTimeout => 504,
            HttpStatusCode::HttpVersionNotSupported => 505,
            HttpStatusCode::VariantAlsoNegotiates => 506,
            HttpStatusCode::InsufficientStorage => 507,
            HttpStatusCode::LoopDetected => 508,
            HttpStatusCode::NotExtended => 510,
            HttpStatusCode::NetworkAuthenticationRequired => 511,
            HttpStatusCode::Custom(code) => *code,
        }
    }

    /// Check if the status code indicates success (2xx)
    pub fn is_success(&self) -> bool {
        let code = self.as_u16();
        code >= 200 && code < 300
    }

    /// Check if the status code indicates a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        let code = self.as_u16();
        code >= 400 && code < 500
    }

    /// Check if the status code indicates a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        let code = self.as_u16();
        code >= 500 && code < 600
    }

    /// Check if the status code indicates an error (4xx or 5xx)
    pub fn is_error(&self) -> bool {
        self.is_client_error() || self.is_server_error()
    }

    /// Check if the status code indicates a redirection (3xx)
    pub fn is_redirection(&self) -> bool {
        let code = self.as_u16();
        code >= 300 && code < 400
    }

    /// Check if the status code indicates an informational response (1xx)
    pub fn is_informational(&self) -> bool {
        let code = self.as_u16();
        code >= 100 && code < 200
    }
}

impl From<reqwest::StatusCode> for HttpStatusCode {
    fn from(status: reqwest::StatusCode) -> Self {
        let code = status.as_u16();
        match code {
            100 => HttpStatusCode::Continue,
            101 => HttpStatusCode::SwitchingProtocols,
            102 => HttpStatusCode::Processing,
            103 => HttpStatusCode::EarlyHints,
            200 => HttpStatusCode::Ok,
            201 => HttpStatusCode::Created,
            202 => HttpStatusCode::Accepted,
            203 => HttpStatusCode::NonAuthoritativeInformation,
            204 => HttpStatusCode::NoContent,
            205 => HttpStatusCode::ResetContent,
            206 => HttpStatusCode::PartialContent,
            207 => HttpStatusCode::MultiStatus,
            208 => HttpStatusCode::AlreadyReported,
            226 => HttpStatusCode::ImUsed,
            300 => HttpStatusCode::MultipleChoices,
            301 => HttpStatusCode::MovedPermanently,
            302 => HttpStatusCode::Found,
            303 => HttpStatusCode::SeeOther,
            304 => HttpStatusCode::NotModified,
            305 => HttpStatusCode::UseProxy,
            307 => HttpStatusCode::TemporaryRedirect,
            308 => HttpStatusCode::PermanentRedirect,
            400 => HttpStatusCode::BadRequest,
            401 => HttpStatusCode::Unauthorized,
            402 => HttpStatusCode::PaymentRequired,
            403 => HttpStatusCode::Forbidden,
            404 => HttpStatusCode::NotFound,
            405 => HttpStatusCode::MethodNotAllowed,
            406 => HttpStatusCode::NotAcceptable,
            407 => HttpStatusCode::ProxyAuthenticationRequired,
            408 => HttpStatusCode::RequestTimeout,
            409 => HttpStatusCode::Conflict,
            410 => HttpStatusCode::Gone,
            411 => HttpStatusCode::LengthRequired,
            412 => HttpStatusCode::PreconditionFailed,
            413 => HttpStatusCode::PayloadTooLarge,
            414 => HttpStatusCode::UriTooLong,
            415 => HttpStatusCode::UnsupportedMediaType,
            416 => HttpStatusCode::RangeNotSatisfiable,
            417 => HttpStatusCode::ExpectationFailed,
            418 => HttpStatusCode::ImATeapot,
            421 => HttpStatusCode::MisdirectedRequest,
            422 => HttpStatusCode::UnprocessableEntity,
            423 => HttpStatusCode::Locked,
            424 => HttpStatusCode::FailedDependency,
            425 => HttpStatusCode::TooEarly,
            426 => HttpStatusCode::UpgradeRequired,
            428 => HttpStatusCode::PreconditionRequired,
            429 => HttpStatusCode::TooManyRequests,
            431 => HttpStatusCode::RequestHeaderFieldsTooLarge,
            451 => HttpStatusCode::UnavailableForLegalReasons,
            500 => HttpStatusCode::InternalServerError,
            501 => HttpStatusCode::NotImplemented,
            502 => HttpStatusCode::BadGateway,
            503 => HttpStatusCode::ServiceUnavailable,
            504 => HttpStatusCode::GatewayTimeout,
            505 => HttpStatusCode::HttpVersionNotSupported,
            506 => HttpStatusCode::VariantAlsoNegotiates,
            507 => HttpStatusCode::InsufficientStorage,
            508 => HttpStatusCode::LoopDetected,
            510 => HttpStatusCode::NotExtended,
            511 => HttpStatusCode::NetworkAuthenticationRequired,
            _ => HttpStatusCode::Custom(code),
        }
    }
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}
