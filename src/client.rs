use reqwest::header::{ACCEPT, USER_AGENT};

use crate::{error::TranslateError, response::parse_google_translate_response};

pub const GOOGLE_TRANSLATE_ENDPOINT: &str = "https://translate.google.com/translate_a/single";

const GOOGLE_TRANSLATE_CLIENT: &str = "gtx";
const GOOGLE_TRANSLATE_USER_AGENT: &str = concat!(
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ",
    "AppleWebKit/537.36 (KHTML, like Gecko) ",
    "Chrome/125.0.0.0 Safari/537.36"
);

#[derive(Debug, Clone)]
pub struct GoogleTranslator {
    client: reqwest::Client,
}

impl GoogleTranslator {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn translate_async(
        &self,
        text: &str,
        source_language: &str,
        target_language: &str,
    ) -> Result<String, TranslateError> {
        let response_body = self
            .client
            .get(GOOGLE_TRANSLATE_ENDPOINT)
            .header(USER_AGENT, GOOGLE_TRANSLATE_USER_AGENT)
            .header(ACCEPT, "application/json, text/javascript, */*; q=0.01")
            .query(&google_translate_request_params(
                text,
                source_language,
                target_language,
            ))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        parse_google_translate_response(&response_body)
    }
}

impl Default for GoogleTranslator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn google_translate_request_params<'a>(
    text: &'a str,
    source_language: &'a str,
    target_language: &'a str,
) -> [(&'static str, &'a str); 5] {
    let source_language = if source_language.is_empty() {
        "auto"
    } else {
        source_language
    };

    [
        ("client", GOOGLE_TRANSLATE_CLIENT),
        ("sl", source_language),
        ("tl", target_language),
        ("dt", "t"),
        ("q", text),
    ]
}

#[cfg(test)]
mod tests {
    use super::{GOOGLE_TRANSLATE_ENDPOINT, google_translate_request_params};

    #[test]
    fn request_params_use_google_translate_endpoint_conventions() {
        let params = google_translate_request_params("Hello, world", "en", "zh-CN");

        assert_eq!(
            GOOGLE_TRANSLATE_ENDPOINT,
            "https://translate.google.com/translate_a/single"
        );
        assert_eq!(
            params,
            [
                ("client", "gtx"),
                ("sl", "en"),
                ("tl", "zh-CN"),
                ("dt", "t"),
                ("q", "Hello, world"),
            ]
        );
    }

    #[test]
    fn request_params_convert_empty_source_language_to_auto_detection() {
        let params = google_translate_request_params("Good morning", "", "ja");

        assert_eq!(params[1], ("sl", "auto"));
    }
}
