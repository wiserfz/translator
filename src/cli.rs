use clap::Parser;

use crate::language::{DEFAULT_SOURCE_LANGUAGE, DEFAULT_TARGET_LANGUAGE, normalize_language_code};

/// Translate terminal input from one language to another using Google Translate.
#[derive(Debug, Parser)]
#[command(
    name = "tror",
    version,
    about = "Translate terminal text with Google Translate",
    long_about = "Translate one or more text fragments from a source language to a target language.\n\nBy default, tror translates English text to Simplified Chinese. Pass multiple text fragments to preserve line breaks in the translated input.",
    after_help = "Examples:\n  tror \"Hello, world\" \"this is Rust code language.\"\n  tror -i cn -o en \"你好，世界；这是 Rust 编程语言\"\n  tror -i auto -o ja \"Good morning\"",
    styles = clap_cargo::style::CLAP_STYLING
)]
pub struct Cli {
    /// Source language code. Use "auto" to let Google Translate detect it.
    #[arg(
        short = 'i',
        long = "input",
        value_name = "LANG",
        default_value = DEFAULT_SOURCE_LANGUAGE,
        help = "Source language code, for example: en, cn, zh, ja, or auto"
    )]
    pub input_language: String,

    /// Target language code.
    #[arg(
        short = 'o',
        long = "output",
        value_name = "LANG",
        default_value = DEFAULT_TARGET_LANGUAGE,
        help = "Target language code, for example: zh-CN, en, or ja"
    )]
    pub output_language: String,

    /// Text fragments to translate. Multiple fragments are joined with newlines.
    #[arg(
        required = true,
        value_name = "TEXT",
        help = "Text fragments to translate; multiple fragments are joined with newlines"
    )]
    pub text: Vec<String>,
}

impl Cli {
    pub fn input_text(&self) -> String {
        self.text.join("\n")
    }

    pub fn source_language(&self) -> String {
        normalize_language_code(&self.input_language)
    }

    pub fn target_language(&self) -> String {
        normalize_language_code(&self.output_language)
    }
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use clap::{CommandFactory, Parser};

    use super::Cli;
    use crate::language::{DEFAULT_SOURCE_LANGUAGE, DEFAULT_TARGET_LANGUAGE};

    #[test]
    fn parses_default_languages_and_single_text() {
        let cli = Cli::parse_from(["tror", "Hello, world"]);

        assert_eq!(cli.input_language, DEFAULT_SOURCE_LANGUAGE);
        assert_eq!(cli.output_language, DEFAULT_TARGET_LANGUAGE);
        assert_eq!(cli.input_text(), "Hello, world");
    }

    #[test]
    fn parses_custom_languages_and_multiple_text_fragments() {
        let cli = Cli::parse_from([
            "tror",
            "-i",
            "cn",
            "-o",
            "en",
            "你好，世界",
            "这是 Rust 编程语言",
        ]);

        assert_eq!(cli.source_language(), "zh-CN");
        assert_eq!(cli.target_language(), "en");
        assert_eq!(cli.input_text(), "你好，世界\n这是 Rust 编程语言");
    }

    #[test]
    fn help_includes_examples_and_language_guidance() {
        let mut command = Cli::command();
        let help = command.render_long_help().to_string();

        assert!(help.contains("Translate one or more text fragments"));
        assert!(help.contains("Examples:"));
        assert!(help.contains("tror -i cn -o en"));
        assert!(help.contains("Source language code"));
        assert!(help.contains("Target language code"));
    }

    #[test]
    fn command_uses_clap_cargo_styles() {
        let command = Cli::command();

        assert_eq!(
            format!("{:?}", command.get_styles()),
            format!("{:?}", clap_cargo::style::CLAP_STYLING)
        );
    }
}
