use clap::Parser;
use confy;
use serde::{Deserialize, Serialize};
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
    word: Option<String>,

    /// Show phonetic pronounciation information
    #[clap(long, short, action)]
    phonetic: bool,

    /// enable local definition caching
    #[clap(long, action)]
    enable_caching: bool,

    /// disable local definition caching
    #[clap(long, action)]
    disable_caching: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TonguesConfig {
    caching: bool,
}

/// `TonguesConfig` implements `Default`
impl ::std::default::Default for TonguesConfig {
    fn default() -> Self {
        Self { caching: true }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let cfg: TonguesConfig = confy::load("tongues", None)?;
    println!("{:?}", cfg);
    if args.enable_caching && args.disable_caching {
        eprintln!("Error: you cannot enable and disable caching simultaneously");
        std::process::exit(1);
    } else if args.enable_caching {
        let cfg = TonguesConfig {
            caching: true,
            ..cfg
        };
        confy::store("tongues", None, &cfg)?;
        println!("Caching enabled");
        std::process::exit(0);
    } else if args.disable_caching {
        let cfg = TonguesConfig {
            caching: false,
            ..cfg
        };
        confy::store("tongues", None, &cfg)?;
        println!("Caching disabled");
        std::process::exit(0);
    }

    let word: String = match args.word {
        Some(word) => word,
        None => {
            eprintln!("Error: you have not specified a word to define");
            std::process::exit(1);
        }
    };

    let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/en/") + &word;
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
