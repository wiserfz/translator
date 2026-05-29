pub const DEFAULT_SOURCE_LANGUAGE: &str = "en";
pub const DEFAULT_TARGET_LANGUAGE: &str = "zh-CN";

pub fn normalize_language_code(language: &str) -> String {
    match language.trim() {
        "auto" => String::new(),
        "cn" | "zh" => "zh-CN".to_owned(),
        normalized => normalized.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_language_code;

    #[test]
    fn normalizes_auto_source_language_to_empty_string_for_google_translate() {
        assert_eq!(normalize_language_code("auto"), "");
    }

    #[test]
    fn normalizes_chinese_aliases_to_simplified_chinese() {
        assert_eq!(normalize_language_code("cn"), "zh-CN");
        assert_eq!(normalize_language_code("zh"), "zh-CN");
    }

    #[test]
    fn trims_language_code_whitespace() {
        assert_eq!(normalize_language_code(" en "), "en");
    }
}
