use clap::Parser;
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashMap;

/// CLI tool to get random jokes in different languages from an embedded Markdown file
#[derive(Parser)]
#[command(name = "joke_cli")]
#[command(about = "A simple CLI that tells jokes in different languages from an embedded markdown file.")]
struct Args {
    /// Language code for the joke (e.g., 'en', 'es', 'fr')
    #[arg(short, long)]
    language: String,
}

/// Parse jokes from a Markdown string.
/// The format is:
/// ## language_code
/// - joke 1
/// - joke 2
fn parse_jokes_from_md(contents: &str) -> HashMap<String, Vec<String>> {
    let mut jokes = HashMap::new();
    let re = Regex::new(r"(?m)^##\s*(\w+)\s*$").unwrap();

    let mut current_lang = None;
    for line in contents.lines() {
        if let Some(caps) = re.captures(line) {
            current_lang = Some(caps[1].to_string());
        } else if let Some(lang) = &current_lang {
            if line.starts_with("- ") {
                let joke = line[2..].trim().to_string();
                jokes.entry(lang.clone()).or_insert_with(Vec::new).push(joke);
            }
        }
    }

    jokes
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Embed the jokes.md content directly
    let jokes_md_content = include_str!("../resources/jokes.md");

    // Parse the jokes from the markdown content
    let jokes = parse_jokes_from_md(jokes_md_content);

    // Check if the language exists in the parsed data
    if let Some(language_jokes) = jokes.get(&args.language) {
        // Choose a random joke
        if let Some(joke) = language_jokes.choose(&mut rand::thread_rng()) {
            println!("{}", joke);
        } else {
            println!("No jokes available in the '{}' language.", args.language);
        }
    } else {
        println!("Language '{}' not found in the jokes file.", args.language);
    }

    Ok(())
}
