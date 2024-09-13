use clap::Parser;
use jokes_rust::{get_jokes_by_language, get_random_joke};

/// A simple CLI for random jokes in English or Portuguese
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Language to get the joke in (English or Portuguese)
    #[arg(short, long, default_value = "english")]
    language: String,
}

fn main() {
    let args = Args::parse();

    // Select the jokes based on the command-line argument
    match get_jokes_by_language(&args.language.to_lowercase()) {
        Some(jokes) => {
            let random_joke = get_random_joke(jokes);
            println!("{}\n{}", random_joke.setup, random_joke.punchline);
        }
        None => {
            println!("Invalid language. Please choose either 'English' or 'Portuguese'.");
        }
    }
}
