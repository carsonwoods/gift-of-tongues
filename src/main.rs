use clap::Parser;
use confy;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

// handles teh arguments that can be passed into the executable
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

// basic settings struct, used to store values from confy
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
    // read arugments and config files
    let args = Args::parse();
    let cfg: TonguesConfig = confy::load("tongues", None)?;

    // gets the path of the config file and ensures that it can be accessed
    let mut cfg_path = match confy::get_configuration_file_path("tongues", None) {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Error: cannot find configuration directory, caching will be disabled");
            let cfg = TonguesConfig {
                caching: false,
                ..cfg
            };
            confy::store("tongues", None, &cfg)?;
            std::process::exit(1);
        }
    };

    // gets the path of the config file and ensures that its parent directory can be accessed
    // for the sake of caching
    cfg_path = match cfg_path.parent() {
        Some(path) => path.to_path_buf(),
        None => {
            eprintln!("Error: cannot find configuration directory, caching will be disable");
            let cfg = TonguesConfig {
                caching: false,
                ..cfg
            };
            confy::store("tongues", None, &cfg)?;
            std::process::exit(1);
        }
    };

    // handles the enabling/disabling of caching of dictionary data
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

    // checks to see if a word was correctly passed into the executable
    let word: String = match args.word {
        Some(word) => word.clone(),
        None => {
            eprintln!("Error: you have not specified a word to define");
            std::process::exit(1);
        }
    };

    let data;

    // if caching is enabled, get the data from cache (if available), otherwise use API
    if cfg.caching {
        let cache_path = cfg_path
            .join("cache")
            .join(&word)
            .join(word.clone() + ".json");
        if cache_path.exists() {
            let file_data = fs::read_to_string(cache_path).expect("Unable to read file");
            data = serde_json::from_str(&file_data).expect("JSON was not well-formatted");
        } else {
            // if the previous file does not exist, attempt to create a new directory for the data
            std::fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
            let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/en/") + &word;
            data = reqwest::get(url).await?.json::<serde_json::Value>().await?;
            // Save the JSON structure into the other file.
            std::fs::write(cache_path, serde_json::to_string_pretty(&data).unwrap()).unwrap();
        }
    } else {
        let url = String::from("https://api.dictionaryapi.dev/api/v2/entries/en/") + &word;
        data = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    }

    // gets word from json data
    let word = data
        .get(0)
        .and_then(|value| value.get("word"))
        .and_then(|value| value.as_str())
        .unwrap_or("Error: word not found");

    // gets definition from json data
    let definition = data
        .get(0)
        .and_then(|value| value.get("meanings"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("definitions"))
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("definition"))
        .and_then(|value| value.as_str())
        .unwrap_or("Error: no definition found");

    // prints word and definition
    println!("{}: {}", word, definition);

    // print phonetics if requested
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
