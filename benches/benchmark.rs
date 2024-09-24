use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use regex::Regex;

/// The function to parse jokes from the markdown file
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

/// The content of a sample markdown file for benchmarking
const TEST_MARKDOWN: &str = r#"
## en
- Why don't scientists trust atoms? Because they make up everything!
- I told my wife she was drawing her eyebrows too high. She looked surprised.

## es
- ¿Por qué los pájaros no usan Facebook? Porque ya tienen Twitter.
- ¿Cómo organizas una fiesta en el espacio? ¡Le das un planetazo!
"#;

/// Benchmark function
fn benchmark_parsing(c: &mut Criterion) {
    c.bench_function("parse_jokes_from_md", |b| {
        b.iter(|| parse_jokes_from_md(black_box(TEST_MARKDOWN)))
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
