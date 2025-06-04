
use serde_json::Value;
use regex::Regex;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub async fn translate_text(input: &str, from: Option<&str>, to: Option<&str>) -> Result<String, Box<dyn std::error::Error>>{
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


fn strip_discord_emojis(input: &str) -> String {
	let re = Regex::new(r"<a?:([a-zA-Z0-9_]+):\d+>").unwrap();
	re.replace_all(input, "$1 ").to_string()
}
