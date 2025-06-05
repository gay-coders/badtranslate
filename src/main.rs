mod translate;
use std::{collections::HashMap, fs::File, io};

use crate::translate::{bt_random_run, bt_run};
use translate::bt_normal_translate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_json: File = File::open("src/lang-list.jsonc").expect("Couldn't open json file");
    let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
    let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;

    let sample_text: &str = "real";

    let mut response: String;

    response = bt_normal_translate(sample_text, Some("en"), Some("zh-CN")).await?;
    println!("[Translate FROM English TO Simplified-Chinese]:\n{response}");

    response = bt_run(sample_text, &languages, Some(10)).await?;
    println!("[Translate through the FIRST 10 langs]:\n{response}");

    response = bt_random_run(sample_text, &languages, Some(10)).await?;
    println!("[badtranslate_random with 10 iterations returned]:\n{response}");

    response = bt_run(sample_text, &languages, None).await?;
    println!("[Translate through the ALL langs in order]:\n{response}");

    response = bt_random_run(sample_text, &languages, None).await?;
    println!("[Translate through the ALL langs in random order]:\n{response}");

    Ok(())
}
