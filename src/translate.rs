use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

pub async fn bt_translate(
    input: &str,
    from: &str,
    to: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let to_language: &str = to.unwrap_or("en");
    let input: String = bt_strip_emojis(input);
    let body: Value = reqwest::get(format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl={from}&tl={to_language}&dt=t&q={}", utf8_percent_encode(input.as_str(), NON_ALPHANUMERIC))).await?.json().await?;
    bt_deserialize_json(body)
}

pub async fn bt_run(
    input: &str,
    languages: &BTreeMap<String, String>,
    limit: Option<usize>,
    indef: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut current_translation: String = String::from(input);
    let mut translate_count: usize = 0;
    let translate_limit: usize = limit.unwrap_or(languages.len());

    // translate through all langs (provided in json file)
    // (or till you reach the limit)
    // (google translate has 153 here e.g.)
    if !indef {
        for (code, lang) in languages {
            if translate_count < translate_limit {
                current_translation =
                    bt_translate(&current_translation, "auto", Some(code.as_str())).await?;
                println!("[TRANSLATED TO \"{lang}\"]:\n{current_translation}");
                println!("-------------------------------------------");
                translate_count += 1;
            } else {
                break;
            }
        }
        current_translation =
            bt_translate(&current_translation.to_string(), "auto", Some("en")).await?;
    } else {
        loop {
            for (code, lang) in languages {
                if translate_count < translate_limit {
                    current_translation =
                        bt_translate(&current_translation, "auto", Some(code.as_str())).await?;
                    println!("[TRANSLATED TO \"{lang}\"]:\n{current_translation}");
                    println!("-------------------------------------------");
                    translate_count += 1;
                } else {
                    break;
                }
            }
        }
    }
    Ok(current_translation)
}

pub async fn bt_random_run(
    input: &str,
    languages: &HashMap<String, String>,
    limit: Option<usize>,
    indef: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut current_translation: String = String::from(input);
    let mut translate_count: usize = 0;
    let translate_limit: usize = limit.unwrap_or(languages.len());

    // translate through all langs (provided in json file)
    // (or till you reach the limit)
    // (google translate has 153 here e.g.)
    if !indef {
        for (code, lang) in languages {
            if translate_count < translate_limit {
                current_translation =
                    bt_translate(&current_translation, "auto", Some(code.as_str())).await?;
                println!("[TRANSLATED TO \"{lang}\"]:\n{current_translation}");
                println!("-------------------------------------------");
                translate_count += 1;
            } else {
                break;
            }
        }
        current_translation =
            bt_translate(&current_translation.to_string(), "auto", Some("en")).await?;
    } else {
        loop {
            for (code, lang) in languages {
                if translate_count < translate_limit {
                    current_translation =
                        bt_translate(&current_translation, "auto", Some(code.as_str())).await?;
                    println!("[TRANSLATED TO \"{lang}\"]:\n{current_translation}");
                    println!("-------------------------------------------");
                    translate_count += 1;
                } else {
                    break;
                }
            }
        }
    }
    Ok(current_translation)
}

fn bt_deserialize_json(google_input: Value) -> Result<String, Box<dyn std::error::Error>> {
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

fn bt_strip_emojis(input: &str) -> String {
    let re: Regex = Regex::new(r"<a?:([a-zA-Z0-9_]+):\d+>").unwrap();
    re.replace_all(input, "$1 ").to_string()
}
