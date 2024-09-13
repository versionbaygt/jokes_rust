// Import the module you want to test
use jokes_rust::{get_jokes_by_language, get_random_joke, Joke, ENGLISH_JOKES};

// Add the tests
#[test]
fn test_get_jokes_by_language_english() {
    let jokes = get_jokes_by_language("english").unwrap();
    assert_eq!(jokes.len(), 4);
    assert_eq!(
        jokes[0],
        Joke {
            setup: "Why don't scientists trust atoms?",
            punchline: "Because they make up everything!",
        }
    );
}

#[test]
fn test_get_jokes_by_language_portuguese() {
    let jokes = get_jokes_by_language("portuguese").unwrap();
    assert_eq!(jokes.len(), 4);
    assert_eq!(
        jokes[0],
        Joke {
            setup: "Por que os cientistas não confiam nos átomos?",
            punchline: "Porque eles formam tudo!",
        }
    );
}

#[test]
fn test_get_jokes_by_language_invalid() {
    let jokes = get_jokes_by_language("spanish");
    assert!(jokes.is_none());
}

#[test]
fn test_get_random_joke() {
    let jokes = ENGLISH_JOKES;
    let random_joke = get_random_joke(jokes);
    assert!(random_joke.setup == "Why don't scientists trust atoms?" 
        || random_joke.setup == "Why did the chicken join a band?"
        || random_joke.setup == "What do you call fake spaghetti?"
        || random_joke.setup == "Why don't skeletons fight each other?");
}
