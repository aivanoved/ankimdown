use ankimdown::markdown::util::log_markdown_events;

const SAMPLE_MARKDOWN_DECK: &str = r#"
# Deck name
This is a sample description

## Deck metadata:

- Maps:
    - Description: Meaning

# hello
## Meaning
1. a greeting

## Metadata

- Templates:
    - Simple
    - Reverse
- Autogen:
    - id: 0
"#;

fn main() {
    log_markdown_events(SAMPLE_MARKDOWN_DECK);
}
