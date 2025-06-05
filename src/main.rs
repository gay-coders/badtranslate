mod translate;
use std::{collections::HashMap, fs::File, io};
use translate::bt_translate;

use crate::translate::bt_badtranslate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_lang_list = File::open("src/lang-list.json").expect("Couldn't open json file");
    let json_reader = io::BufReader::new(json_lang_list);
    let json_data: HashMap<String, String> = serde_json::from_reader(json_reader)?;

    let sample_text: &str = "meow this is me, maow maow\n meoas is this being translated at all";

    let mut response: String;

    // original translate function
    response = bt_translate(sample_text, Some("en"), Some("uk")).await?;
    println!("bt_translate(en to uk) returned: {response}\n");
    response.clear();

    // translate to every language and back to english
    response = bt_badtranslate(sample_text, json_data).await?;
    println!("bt_badtranslate returned: {response}");

    Ok(())
}
