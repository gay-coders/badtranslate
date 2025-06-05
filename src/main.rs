mod translate;
use std::{collections::HashMap, fs::File, io};

use crate::translate::{bt_gibber_random, bt_gibber_translate};
use translate::bt_send_translate_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_json: File = File::open("src/lang-list.jsonc").expect("Couldn't open json file");
    let json_reader: io::BufReader<File> = io::BufReader::new(raw_json);
    let languages: HashMap<String, String> = serde_json::from_reader(json_reader)?;

    let sample_text: &str = "After a spot of tea and a quick natter about the weather,\n I popped down to the local pub for a pint of bitter and a cheeky chat with the lads\n before heading home to watch the footie.";

    let mut response: String;

    response = bt_send_translate_request(sample_text, Some("en"), Some("zh-CN")).await?;
    println!("[Translate FROM English TO Simplified-Chinese]:\n{response}");

    response = bt_gibber_translate(sample_text, &languages, Some(10)).await?;
    println!("[Translate through the FIRST 10 langs]:\n{response}");

    response = bt_gibber_random(sample_text, &languages, Some(10)).await?;
    println!("[badtranslate_random with 10 iterations returned]:\n{response}");

    response = bt_gibber_translate(sample_text, &languages, None).await?;
    println!("[Translate through the ALL langs in order]:\n{response}");

    response = bt_gibber_random(sample_text, &languages, None).await?;
    println!("[Translate through the ALL langs in random order]:\n{response}");

    Ok(())
}
