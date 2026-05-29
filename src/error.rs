use std::{error::Error, fmt};

#[derive(Debug)]
pub enum TranslateError {
    Request(reqwest::Error),
    Json(serde_json::Error),
    InvalidResponse(&'static str),
}

impl fmt::Display for TranslateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(error) => write!(formatter, "Google Translate request failed: {error}"),
            Self::Json(error) => {
                write!(formatter, "Google Translate returned invalid JSON: {error}")
            }
            Self::InvalidResponse(message) => {
                write!(
                    formatter,
                    "Google Translate returned an unexpected response: {message}"
                )
            }
        }
    }
}

impl Error for TranslateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Request(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::InvalidResponse(_) => None,
        }
    }
}

impl From<reqwest::Error> for TranslateError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<serde_json::Error> for TranslateError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
