mod translate;
use crate::translate::{bt_random_run, bt_run};
use clap::{Parser, Subcommand};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io,
};
use translate::bt_translate;

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
        #[arg(short)]
        input: String,
        #[arg(short)]
        from: String,
        #[arg(short)]
        to: String,
    },
    #[command(
        about = "Translate to each language (within order of the json file) and then back to english to get the mess"
    )]
    Gibber {
        #[arg(short)]
        input: String,
        #[arg(short)]
        count: usize,
        #[arg(short)]
        ordered: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: CLI = CLI::parse();

    match &cli.command {
        Commands::Translate { input, from, to } => {
            let response = bt_translate(input.as_str(), Some(from), Some(to)).await?;
            println!("\"{input}\" FROM {from} to {to}:\n{response}");
            println!("-------------------------------------------");
        }
        Commands::Gibber {
            input,
            count,
            ordered,
        } => {
            let raw_json: File =
                File::open("src/lang-list.jsonc").expect("Couldn't open json file");
            let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
            if *ordered {
                let languages: BTreeMap<String, String> = serde_json::from_reader(json_reader)?;
                let response = bt_run(input.as_str(), &languages, Some(*count)).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            } else {
                let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;
                let response = bt_random_run(input.as_str(), &languages, Some(*count)).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            }
        }
    }
    Ok(())
}
