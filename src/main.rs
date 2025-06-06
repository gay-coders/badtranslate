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
            help = "This is the original language of your text, by default we set it to (\"auto\")."
        )]
        from: Option<String>,
        #[arg(
            short,
            help = "This is the language you want to translate your text into."
        )]
        to: String,
    },
    #[command(about = "Translate to each language available in json file.")]
    Gibber {
        #[arg(help = "Text to be translated.")]
        input: String,
        #[arg(
            short,
            help = "This is how many times you want to translate. By default it's (10)."
        )]
        count: Option<usize>,
        #[arg(
            short,
            help = "This is if you want to translate through the list json file in order of lines or not."
        )]
        ordered: Option<bool>,
        #[arg(
            short,
            help = "This is if you want to translate through the list json file in order of lines or not."
        )]
        forever: Option<bool>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: BtCli = BtCli::parse();

    match &cli.arguments {
        Options::Translate { input, from, to } => {
            let from_lang: String = from.as_ref().unwrap_or(&"auto".to_string()).clone();
            let response: String =
                bt_translate(input.as_str(), from_lang.as_str(), Some(to)).await?;
            println!("\"{input}\" FROM {from_lang} to {to}:\n{response}");
            println!("-------------------------------------------");
        }
        Options::Gibber {
            input,
            count,
            ordered,
            forever,
        } => {
            let raw_json: File = File::open("src/lang-list.json").expect("Couldn't open json file");
            let json_reader: BufReader<File> = BufReader::new(raw_json);
            let order_or_not: bool = ordered.unwrap_or(false);
            let forever_or_not: bool = forever.unwrap_or(true);
            if order_or_not {
                let languages: BTreeMap<String, String> = serde_json::from_reader(json_reader)?;
                let response: String =
                    bt_run(input.as_str(), &languages, *count, forever_or_not).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            } else {
                let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;
                let response: String =
                    bt_random_run(input.as_str(), &languages, *count, forever_or_not).await?;
                println!("Back to English:\n{response}");
                println!("-------------------------------------------");
            }
        }
    }
    Ok(())
}
