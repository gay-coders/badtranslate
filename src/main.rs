mod translate;
use crate::translate::{bt_random_run, bt_run};
use clap::{Parser, Subcommand};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io,
};
use translate::bt_normal_translate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Translate normally from one language to another")]
    Translate {
        input_string: String,
        from_lang: String,
        to_lang: String,
    },
    #[command(
        about = "Translate to each language (within order of the json file) and then back to english to get the mess"
    )]
    GibberInOrder {
        input_string: String,
        iteration: usize,
    },
    #[command(about = "Translate to each language and then back to english to get the mess")]
    GibberRandom {
        input_string: String,
        iteration: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: CLI = CLI::parse();

    match &cli.command {
        Commands::Translate {
            input_string,
            from_lang,
            to_lang,
        } => {
            let response =
                bt_normal_translate(input_string.as_str(), Some(from_lang), Some(to_lang)).await?;
            println!("{input_string} FROM {from_lang} to {to_lang}:\n{response}");
            println!("-------------------------------------------");
        }
        Commands::GibberInOrder {
            input_string,
            iteration,
        } => {
            let raw_json: File =
                File::open("src/lang-list.jsonc").expect("Couldn't open json file");
            let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
            let languages: BTreeMap<String, String> = serde_json::from_reader(json_reader)?;
            let response = bt_run(input_string.as_str(), &languages, Some(*iteration)).await?;
            println!("Back to English:\n{response}");
            println!("-------------------------------------------");
        }
        Commands::GibberRandom {
            input_string,
            iteration,
        } => {
            let raw_json: File =
                File::open("src/lang-list.jsonc").expect("Couldn't open json file");
            let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
            let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;
            let response =
                bt_random_run(input_string.as_str(), &languages, Some(*iteration)).await?;
            println!("Back to English:\n{response}");
            println!("-------------------------------------------");
        }
    }
    Ok(())
}
