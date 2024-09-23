use std::collections::HashMap;
use rand::seq::SliceRandom;
use regex::Regex;

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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MARKDOWN: &str = r#"
## en
- Why don't scientists trust atoms? Because they make up everything!
- I told my wife she was drawing her eyebrows too high. She looked surprised.

## es
- ¿Por qué los pájaros no usan Facebook? Porque ya tienen Twitter.
- ¿Cómo organizas una fiesta en el espacio? ¡Le das un planetazo!
"#;

    #[test]
    fn test_parse_jokes_from_md() {
        let jokes = parse_jokes_from_md(TEST_MARKDOWN);

        // Check if the languages exist
        assert!(jokes.contains_key("en"));
        assert!(jokes.contains_key("es"));

        // Check if the number of jokes is correct
        assert_eq!(jokes["en"].len(), 2);
        assert_eq!(jokes["es"].len(), 2);

        // Check the content of one joke
        assert_eq!(
            jokes["en"][0],
            "Why don't scientists trust atoms? Because they make up everything!"
        );
    }

    #[test]
    fn test_random_joke_selection() {
        let jokes = parse_jokes_from_md(TEST_MARKDOWN);
        
        // Ensure there's at least one joke in English
        if let Some(english_jokes) = jokes.get("en") {
            let random_joke = english_jokes.choose(&mut rand::thread_rng());
            assert!(random_joke.is_some());
        } else {
            panic!("No jokes found for 'en' language");
        }
    }

    #[test]
    fn test_language_not_found() {
        let jokes = parse_jokes_from_md(TEST_MARKDOWN);
        
        // Check that an unsupported language doesn't exist
        assert!(!jokes.contains_key("de"));
    }
}
