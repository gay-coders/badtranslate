use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub async fn bt_translate(
    input: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let from_language = from.unwrap_or("auto");
    let to_language = to.unwrap_or("en");
    let input = strip_discord_emojis(input);
    let body: Value = reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl={from_language}&tl={to_language}&dt=t&q={}", utf8_percent_encode(input.as_str(), NON_ALPHANUMERIC))).await?.json().await?;

    if let Some(array) = body.get(0).and_then(|v| v.as_array()) {
        let result = array
            .iter()
            .filter_map(|s| s.get(0))
            .filter_map(|v| v.as_str())
            .collect::<String>();

        Ok(result)
    } else {
        Ok(String::new())
    }
}

// you could remove the temp_abom(ination) string variable and just use the input and make it mutable
// this is your preference i didn't touch it

pub async fn bt_badtranslate(
    input: &str,
    json_data: HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut temp_abom: String = String::from(input);
    let mut google_input: String;

    // translate through all langs in the json data (google translate has 153 here e.g.)
    for (code, lang) in json_data {
        google_input = bt_beautify_response(reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={code}&dt=t&q={}", utf8_percent_encode(temp_abom.as_str(), NON_ALPHANUMERIC))).await?.json().await?).unwrap();
        temp_abom = google_input.clone();
        println!("At language {lang}: {temp_abom}");
    }

    google_input = bt_beautify_response(reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=en&dt=t&q={}", utf8_percent_encode(input, NON_ALPHANUMERIC))).await?.json().await?).unwrap();
    println!("Back to english: {google_input}");

    Ok(google_input)
}

fn bt_beautify_response(google_input: Value) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(array) = google_input.get(0).and_then(|v| v.as_array()) {
        let result = array
            .iter()
            .filter_map(|s| s.get(0))
            .filter_map(|v| v.as_str())
            .collect::<String>()
            .trim()
            .to_string();

        Ok(result)
    } else {
        Ok(String::new())
    }
}

fn strip_discord_emojis(input: &str) -> String {
    let re = Regex::new(r"<a?:([a-zA-Z0-9_]+):\d+>").unwrap();
    re.replace_all(input, "$1 ").to_string()
}
