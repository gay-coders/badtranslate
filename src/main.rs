mod translate;
use crate::translate::{bt_random_run, bt_run};
use clap::{Parser, Subcommand};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
};
use translate::bt_translate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct BtCli {
    #[command(subcommand)]
    arguments: Options,
}
#[derive(Subcommand)]
enum Options {
    #[command(about = "Normal Translation from a language to another.")]
    Translate {
        #[arg(help = "Text to be translated.")]
        input: String,
        #[arg(
            short,
            help = "Language of the inputted text. (use the short names from the default JSON).",
						default_value_t = {"auto".to_string()}
        )]
        from: String,
        #[arg(
            short,
            help = "Language to translate inputted text into."
        )]
        to: String,
    },
    #[command(about = "Chain translate between specified languages.")]
    Chain {
        #[arg(help = "Text to be translated.")]
        input: String,
        #[arg(
            long,
            help = "Path to the languages JSON file.",
						default_value_t = {"src/lang-list.json".to_string()}
        )]
        languages_file: String,
        #[arg(
            long,
            help = "How many times to translate.",
						default_value_t = 10,
        )]
        count: usize,
        #[arg(
            long,
            help = "Whether to follow order of the JSON input",
						default_value_t = false
        )]
        ordered: bool,
        #[arg(
            long,
            help = "Whether to keep running it forever in a loop between all the languages. Use Ctrl + C to stop",
						default_value_t = false
        )]
        forever: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: BtCli = BtCli::parse();

    match &cli.arguments {
        Options::Translate { input, from, to } => {
            let response: String =
                bt_translate(input.as_str(), from.as_str(), Some(to)).await?;
            println!("\"{input}\" FROM {from} to {to}:\n{response}");
            println!("-------------------------------------------");
        }
        Options::Chain {
            input,
						languages_file,
            count,
            ordered,
            forever,
        } => {
            let raw_json: File = File::open(languages_file).expect("Couldn't open json file");
            let json_reader: BufReader<File> = BufReader::new(raw_json);
            if *ordered {
                let languages: BTreeMap<String, String> = serde_json::from_reader(json_reader)?;
                let response: String =
                    bt_run(input.as_str(), &languages, Some(*count), *forever).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            } else {
                let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;
                let response: String =
                    bt_random_run(input.as_str(), &languages, Some(*count), *forever).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            }
        }
    }
    Ok(())
}
