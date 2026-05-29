use serde_json::Value;

use crate::error::TranslateError;

pub fn parse_google_translate_response(response_body: &str) -> Result<String, TranslateError> {
    let response: Value = serde_json::from_str(response_body)?;
    let segments =
        response
            .get(0)
            .and_then(Value::as_array)
            .ok_or(TranslateError::InvalidResponse(
                "missing translation segment list",
            ))?;

    let translated_text = segments
        .iter()
        .filter_map(|segment| segment.get(0).and_then(Value::as_str))
        .collect::<String>();

    if translated_text.is_empty() {
        return Err(TranslateError::InvalidResponse(
            "missing translated text segment",
        ));
    }

    Ok(translated_text)
}

#[cfg(test)]
mod tests {
    use super::parse_google_translate_response;

    #[test]
    fn parses_google_translate_response_segments() {
        let response_body =
            r#"[[["你好","Hello",null,null,10],["，世界","，world",null,null,10]],null,"en"]"#;

        let translated_text = parse_google_translate_response(response_body).unwrap();

        assert_eq!(translated_text, "你好，世界");
    }

    #[test]
    fn rejects_google_translate_response_without_segments() {
        let error = parse_google_translate_response("[]").unwrap_err();

        assert!(
            error
                .to_string()
                .contains("missing translation segment list")
        );
    }

    #[test]
    fn rejects_google_translate_response_without_translated_text() {
        let error = parse_google_translate_response("[[[null,\"Hello\"]]]").unwrap_err();

        assert!(
            error
                .to_string()
                .contains("missing translated text segment")
        );
    }
}
