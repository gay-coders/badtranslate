mod translate;
use translate::translate_text;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let response = translate_text("meow this is me, maow maow\n meoas is this being translated at all", Some("auto"), Some("uk")).await?;
	println!("Translated text: {}", response);
	Ok(())
}