use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub async fn translate(
    input: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let from_language: &str = from.unwrap_or("auto");
    let to_language: &str = to.unwrap_or("en");
    let input: String = strip_discord_emojis(input);
    let body: Value = reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl={from_language}&tl={to_language}&dt=t&q={}", utf8_percent_encode(input.as_str(), NON_ALPHANUMERIC))).await?.json().await?;
    gtranslate_json_to_string(body)
}

pub async fn badtranslate(
    input: &str,
    languages: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut current_translation: String = String::from(input);

    // translate through all langs in the json data (google translate has 153 here e.g.)
    for (code, lang) in languages {
        current_translation = translate(&current_translation, Some("auto"), Some(code.as_str()))
            .await
            .unwrap();
        println!("At language {lang}: {current_translation}");
    }
    current_translation = translate(&current_translation.to_string(), Some("auto"), Some("en"))
        .await
        .unwrap(); //bt_beautify_response(reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=en&dt=t&q={}", utf8_percent_encode(input, NON_ALPHANUMERIC))).await?.json().await?).unwrap();
    println!("Back to english: {current_translation}");

    Ok(current_translation)
}

pub async fn badtranslate_random(
    input: &str,
    languages: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    unimplemented!();
}
fn gtranslate_json_to_string(google_input: Value) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(array) = google_input.get(0).and_then(|v: &Value| v.as_array()) {
        Ok(array
            .iter()
            .filter_map(|s| s.get(0))
            .filter_map(|v| v.as_str())
            .collect::<String>()
            .trim()
            .to_string())
    } else {
        Ok(String::new())
    }
}

fn strip_discord_emojis(input: &str) -> String {
    let re: Regex = Regex::new(r"<a?:([a-zA-Z0-9_]+):\d+>").unwrap();
    re.replace_all(input, "$1 ").to_string()
}
