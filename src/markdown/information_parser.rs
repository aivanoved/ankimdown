use crate::ankigen::information::SimpleInformation;
use pulldown_cmark;

use crate::markdown::util::parse_heading;
use crate::markdown::util::{split_at_event, SplitHeadingLevel, SplitStop};

struct SplitListItem {
    pub depth: usize,
}

impl SplitStop<usize> for SplitListItem {
    fn split(&self, event: &pulldown_cmark::Event, state: &Option<usize>) -> (bool, Option<usize>) {
        let current_depth = state.unwrap_or(0);
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::List(..)) => {
                (current_depth == self.depth, Some(current_depth + 1))
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::List(..)) => {
                (false, Some(current_depth - 1))
            }
            _ => (false, Some(current_depth)),
        }
    }
    fn stop(&self, event: &pulldown_cmark::Event, state: &Option<usize>) -> bool {
        let current_depth = state.unwrap_or(0);
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::List(..)) => {
                current_depth < self.depth
            }
            _ => false,
        }
    }
}

pub fn parse_information(input: &str) -> Result<SimpleInformation, String> {
    let mut parser = pulldown_cmark::Parser::new_ext(input, pulldown_cmark::Options::all());

    let information = SimpleInformation::default();

    let heading_level = split_at_event(
        &mut parser,
        SplitHeadingLevel {
            level: pulldown_cmark::HeadingLevel::H1,
        },
    );

    println!("Heading Level: {:?}", heading_level);

    if heading_level.is_empty() {
        return Err("No headings found".to_string());
    } else if heading_level.len() > 1 {
        return Err("Multiple headings found".to_string());
    }
    let mut heading = heading_level[0].clone().into_iter();

    let (level, text) = parse_heading(&mut heading)?;

    if level != pulldown_cmark::HeadingLevel::H2 {
        return Err("First heading must be H2".to_string());
    }
    if text != "Definition" {
        return Err("First heading must be Definition".to_string());
    }

    let definitions = split_at_event(
        &mut heading.collect::<Vec<_>>().into_iter(),
        SplitListItem { depth: 0 },
    );

    println!("Definitions: {:?}", definitions);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INFORMATION: &str = r#"
# hello
## Definition
1. a greeting
"#;

    #[test]
    fn test_parse_information() {
        let result = parse_information(EXAMPLE_INFORMATION);
        assert!(result.is_ok());
        let information = result.unwrap();
        assert_eq!(information.word, "hello");
        assert_eq!(information.definitions, vec!["a greeting"]);
    }
}
