use rand::seq::SliceRandom;
use rand::thread_rng;

/// Joke struct to hold setup and punchline
#[derive(Debug, PartialEq)]
pub struct Joke {
    pub setup: &'static str,
    pub punchline: &'static str,
}

// Jokes in English
pub const ENGLISH_JOKES: &[Joke] = &[
    Joke {
        setup: "Why don't scientists trust atoms?",
        punchline: "Because they make up everything!",
    },
    Joke {
        setup: "Why did the chicken join a band?",
        punchline: "Because it had the drumsticks!",
    },
    Joke {
        setup: "What do you call fake spaghetti?",
        punchline: "An impasta!",
    },
    Joke {
        setup: "Why don't skeletons fight each other?",
        punchline: "They don't have the guts.",
    },
];

// Jokes in Portuguese
pub const PORTUGUESE_JOKES: &[Joke] = &[
    Joke {
        setup: "Por que os cientistas não confiam nos átomos?",
        punchline: "Porque eles formam tudo!",
    },
    Joke {
        setup: "Por que a galinha entrou para uma banda?",
        punchline: "Porque ela tinha os baquetas!",
    },
    Joke {
        setup: "Como se chama macarrão falso?",
        punchline: "Uma impasta!",
    },
    Joke {
        setup: "Por que os esqueletos não brigam entre si?",
        punchline: "Eles não têm coragem.",
    },
];

/// Function to return jokes based on the selected language
pub fn get_jokes_by_language(language: &str) -> Option<&'static [Joke]> {
    match language {
        "english" => Some(ENGLISH_JOKES),
        "portuguese" => Some(PORTUGUESE_JOKES),
        _ => None,
    }
}

/// Function to select a random joke from the list
pub fn get_random_joke(jokes: &[Joke]) -> &Joke {
    let mut rng = thread_rng();
    jokes.choose(&mut rng).unwrap()
}
