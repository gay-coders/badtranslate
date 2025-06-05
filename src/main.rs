mod translate;
use std::{collections::HashMap, fs::File, io};

use translate::translate;
use crate::translate::{badtranslate, badtranslate_random};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_json: File = File::open("src/lang-list.json").expect("Couldn't open json file");
    let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
    let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;

    let sample_text: &str = "meow this is me, maow maow\n miaouw is this being translated at all";

    let mut response: String;

    // original translate function
    response = translate(sample_text, Some("en"), Some("uk")).await?;
    println!("translate(en to uk) returned: {response}\n");
    response.clear();

    // translate to every language and back to english
    response = badtranslate(sample_text, &languages).await?;
    println!("badtranslate returned: {response}");

    println!("badtranslate_random with 10 iterations returned: {}", badtranslate_random(sample_text, &languages).await?);
    Ok(())
}
