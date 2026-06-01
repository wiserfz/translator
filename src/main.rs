mod cli;
mod client;
mod error;
mod language;
mod response;

use std::error::Error;

use cli::parse_cli;
use client::GoogleTranslator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = parse_cli();
    let translator = match cli.proxy_url() {
        Some(proxy_url) => GoogleTranslator::with_proxy(Some(proxy_url))?,
        None => GoogleTranslator::new()?,
    };
    let translated_text = translator
        .translate_async(
            &cli.input_text(),
            &cli.source_language(),
            &cli.target_language(),
        )
        .await?;

    println!("{translated_text}");

    Ok(())
}
