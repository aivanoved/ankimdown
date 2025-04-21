use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const EXAMPLE_DECK: &str = r#"
# Deck name
This is a sample description

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
