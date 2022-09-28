use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Dictionary data is provided by: https://api.dictionaryapi.dev"
)]
/// Dictionary Lookup Tool
struct Args {
    /// word to lookup the definition for
    word: String,

    /// Show phonetic pronounciation information
    #[clap(long, short, action)]
    phonetic: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/en/") + &args.word;

    let data = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    let word = data
        .get(0)
        .and_then(|value| value.get("word"))
        .and_then(|value| value.as_str())
        .unwrap_or("Error: word not found");

    let definition = data
        .get(0)
        .and_then(|value| value.get("meanings"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("definitions"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("definition"))
        .and_then(|value| value.as_str())
        .unwrap_or("Error: no definition found");

    println!("{}: {}", word, definition);

    if args.phonetic {
        println!(
            "Phonetic: {:?}",
            data.get(0)
                .and_then(|value| value.get("phonetic"))
                .and_then(|value| value.as_str())
                .unwrap_or("phonetic information not found")
        );
    }

    Ok(())
}
