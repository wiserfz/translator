# Translator Roadmap

Planned and proposed features for the `tror` CLI. Items are grouped by effort and not strictly ordered; pull a card off any group when it makes sense.

## Quick wins

- **Shell completions** — `bash`, `zsh`, `fish`, `powershell`, `elvish` via `clap_complete`.
- **Stdin support** — when no positional `TEXT` is given and stdin is piped, read text from stdin (`echo "hi" | tror`).
- **Verbosity flags** — `--verbose` / `--quiet` for diagnostics.
- **Polished errors** — friendly stderr messages for network, invalid language code, and non-2xx HTTP responses (preserve underlying chain for `--verbose`).

## Medium

- **`-l, --list-languages`** — print supported language codes and exit.
- **Detected-language display** — when `-i auto` is used, show the language Google Translate detected.
- **`--json` output mode** — emit `{ "source": ..., "target": ..., "detected": ..., "translated": ... }`.
- **Config file** — `~/.config/tror/config.toml` for default `-i` / `-o` / `-p`.
- **Retry with backoff** — handle transient network failures.
- **Batch mode** — `--each-line` to translate each input line independently.

## Larger

- **Provider abstraction** — trait + alternative backends (DeepL, LibreTranslate) behind a stable interface.
- **Local cache** — SQLite or JSON cache keyed on `(source, target, text)` to skip repeat calls.
- **Integration tests** — `tests/` directory using `mockall` + `wiremock` for HTTP-level fixtures.
- **CI workflow** — `.github/workflows/` running `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- **Manpage generation** — `clap_mangen` output for packaging.

## Status

| Feature | Status |
| --- | --- |
| Shell completions | Done |
| Stdin support | Done |
| Everything else | Proposed |
