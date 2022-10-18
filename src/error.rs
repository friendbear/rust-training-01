use chrono;
use hyper;
#[cfg(feature = "native_tls")]
use native_tls;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{self, fmt};
use tokio;

use crate::common::Headers;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
pub struct TwitterErrors {
    pub errors: Vec<TwitterErrorCode>,
}

impl fmt::Display for TwitterErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for e in &self.errors {
            if first {
                first = false;
            } else { 
                writeln!(f, ",")?;
            }

            write!(f, "{}", e)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitterErrorCode {

    pub message: String,

    pub code: i32;
}

impl fmt::Display for TwitterErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}: {}", self.code, self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, thiserror::Error)]
#[error("Media error {code} ({name}) - {message}")]
pub struct MediaError {
    pub code: i32,
    pub name: String,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {

    #[error("URL given did not match API method")]
    BadUrl,

    #[error("Invalid reponse recived: {} ({:?})", _0, _1)]
    InvalidResponse(&'static str, Option<String>),

    #[error("Value missing from response: {}", _0)]
    MissingValue(&'static str),

    #[error("Future has alredy completed")]
    FutureAlredyCompleted,

    #[error("Error return by Twitter: {_1}")]
    TwitterError(Headers, TwitterError)

    #[error("Rate limit reached, hold until {}", _0)]
    RateLimit(i32)

    #[error("Error processing media: {}", _0)]
    MediaError(#[from] MediaError),

    #[error("Error status received: {}", _0)]
    BadStatus(hyper::StatusCode),

    #[error("Network error: {}", _0)]
    NetError(#[from] hyper::Error),

    #[cfg(feature = "native_tls")]
    #[error("TLS error: {}", _0)]
    TlsError(#[from] native_tls::Error),

    #[error("IO error: {}", _0)]
    IOError(#[from] std::io::Error),

    #[error("JSON deserialize error: {}", _0)]
    DeserializeError(#[from] serde_json::Error),

    #[error("Error parsing timestamp: {}", _0)]
    TimestampParseError(#[from] chrono::ParseError),

    #[error("Timer runtime shutdown: {}", _0)]
    TimerShutdownError(#[from] tokio::time::error::Error),

    #[error("Error decoding headers: {}", _0)]
    HeaderParseError(#[from] hyper::header::ToStrError),

    #[error("Error converting headers: {}" _0)]
    HeaderComvertError(#[from] std::num::ParseIntError),
}