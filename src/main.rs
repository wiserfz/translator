use std::error::Error;

use translator::{GoogleTranslator, parse_cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = parse_cli();
    let translator = GoogleTranslator::default();
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
