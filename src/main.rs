mod cli;
mod client;
mod error;
mod language;
mod response;

use std::error::Error;
use std::io::{self, IsTerminal};
use std::process::ExitCode;

use clap::CommandFactory;
use clap_complete::generate;

use cli::{Cli, parse_cli, read_text_from_reader};
use client::GoogleTranslator;

#[tokio::main]
async fn main() -> Result<ExitCode, Box<dyn Error>> {
    let cli = parse_cli();

    if let Some(shell) = cli.completions {
        let mut command = Cli::command();
        let bin_name = command.get_name().to_string();
        generate(shell, &mut command, bin_name, &mut io::stdout());
        return Ok(ExitCode::SUCCESS);
    }

    let input_text = resolve_input_text(&cli)?;
    let Some(input_text) = input_text else {
        let mut command = Cli::command();
        eprintln!("error: no text to translate. Pass TEXT as arguments or pipe input via stdin.\n");
        command.print_help()?;
        eprintln!();
        return Ok(ExitCode::from(2));
    };

    let translator = match cli.proxy_url() {
        Some(proxy_url) => GoogleTranslator::with_proxy(Some(proxy_url))?,
        None => GoogleTranslator::new()?,
    };
    let translated_text = translator
        .translate_async(&input_text, &cli.source_language(), &cli.target_language())
        .await?;

    println!("{translated_text}");

    Ok(ExitCode::SUCCESS)
}

fn resolve_input_text(cli: &Cli) -> io::Result<Option<String>> {
    if !cli.text.is_empty() {
        return Ok(Some(cli.input_text()));
    }

    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Ok(None);
    }

    let text = read_text_from_reader(stdin.lock())?;
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}
