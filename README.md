# Translator

Translator is a terminal CLI for translating input text from one language to another. The published binary name is `tror`, while the Rust package name is `translator`.

By default, `tror` translates English text to Simplified Chinese. Source and target languages can be configured with CLI flags.

## Features

- Translate one or more text fragments from the terminal.
- Preserve multiple positional text arguments as newline-separated input.
- Configure source and target languages with short or long flags.
- Use `auto` to ask Google Translate to detect the source language.
- Normalize common aliases such as `cn` and `zh` to `zh-CN`.
- Use an internal async Google Translate web client built on `reqwest` and `tokio`.

## Installation

Build the CLI from the repository root:

```bash
cargo build --release
```

The optimized binary will be available at:

```text
target/release/tror
```

You can also run it directly during development:

```bash
cargo run -- "Hello, world"
```

## Usage

```bash
# Default: English to Simplified Chinese
tror "Hello, world" \
    "this is Rust code language."

# Chinese to English; `cn` is normalized to `zh-CN`
tror -i cn -o en "你好，世界；这是 Rust 编程语言"

# Ask Google Translate to detect the source language
tror -i auto -o ja "Good morning"
```

Show the generated help text:

```bash
tror --help
```

When running through Cargo, pass CLI arguments after `--`:

```bash
cargo run -- -i cn -o en "你好，世界；这是 Rust 编程语言"
```

## CLI Options

- `-i, --input <LANG>`: Source language code. Defaults to `en`.
- `-o, --output <LANG>`: Target language code. Defaults to `zh-CN`.
- `<TEXT>...`: Required text fragments to translate.

Language normalization currently includes:

- `auto`: use automatic source-language detection.
- `cn` and `zh`: normalize to `zh-CN`.

## Development

This project is written in Rust and uses:

- `tokio` for async runtime support.
- `clap` and `clap-cargo` for CLI parsing and styling.
- `reqwest` for HTTP requests.
- `serde_json` for parsing Google Translate response payloads.

Common commands:

```bash
# Build the binary
cargo build

# Run unit tests
cargo test

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Run Clippy with warnings denied
cargo clippy --all-targets --all-features -- -D warnings
```

Recommended verification before handing off Rust changes:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Testing Guidance

Prefer tests that do not require live network access. Request construction, language normalization, CLI parsing, and response parsing should be covered with unit tests or local fixtures.

Avoid relying on real Google Translate calls in automated tests because the service is external and its undocumented response shape can change.

## Coding Standards

- Follow the official Rust style guide: <https://doc.rust-lang.org/style-guide/>.
- Prefer type-safe, idiomatic Rust.
- Keep CLI behavior clear and well documented.
- Use `Result` and `?` for fallible operations instead of panics in production code.
- Keep comments and documentation in English.
- Do not add dependencies unless they clearly improve maintainability, UX, correctness, or testability.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Notes

Runtime translation depends on network access and Google Translate's web endpoint. Failures from the network or response parsing are surfaced through the CLI error path.
