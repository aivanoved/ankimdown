use pulldown_cmark;
use sha2::digest::consts::False;

pub fn log_markdown_events(text: &str) {
    let mut width = 0;
    eprintln!("{text:?}: [");
    for event in pulldown_cmark::Parser::new(text) {
        if let pulldown_cmark::Event::End(_) = event {
            width -= 2;
        }
        eprintln!("    {:width$}{event:?}", "");
        if let pulldown_cmark::Event::Start(_) = event {
            width += 2;
        }
    }
    eprintln!("]");
}

pub struct MarkdownOptions {
    pub indentation: usize,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        MarkdownOptions { indentation: 4 }
    }
}

impl MarkdownOptions {
    pub fn new(indentation: usize) -> Self {
        MarkdownOptions { indentation }
    }
}

pub fn events_to_markdown(
    events: Vec<pulldown_cmark::Event>,
    opts: Option<MarkdownOptions>,
) -> String {
    let markdown_opts = opts.unwrap_or_default();
    let mut output = String::new();
    let mut list_level = 0;
    for event in events {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level, .. }) => {
                output.push_str(&"#".repeat(level as usize));
                output.push(' ');
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(_)) => {
                output.push('\n');
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Paragraph) => {
                if !output.ends_with('\n') {
                    output.push('\n');
                }
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Paragraph) => {
                output.push('\n');
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::List(count)) => {
                let prefix = if count.is_none() {
                    format!(
                        "{:indentation$}",
                        "- ",
                        indentation = markdown_opts.indentation
                    )
                } else {
                    format!(
                        "{:indentation$}{} ",
                        "",
                        count.unwrap(),
                        indentation = list_level * markdown_opts.indentation,
                    )
                };
                if !output.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str(&prefix);
                list_level += 1;
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::List(_)) => {
                list_level -= 1;
                output.push('\n');
            }
            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                output.push('\n');
            }
            pulldown_cmark::Event::Text(ref t) | pulldown_cmark::Event::Code(ref t) => {
                output.push_str(t);
            }
            _ => {}
        }
    }
    output
}

#[derive(Clone, Debug)]
pub struct SplitHeading<'a> {
    pub level: pulldown_cmark::HeadingLevel,
    pub text: String,
    pub events: Vec<pulldown_cmark::Event<'a>>,
}

impl<'a> SplitHeading<'a> {
    pub fn new(
        level: pulldown_cmark::HeadingLevel,
        text: String,
        events: Vec<pulldown_cmark::Event<'a>>,
    ) -> Self {
        SplitHeading {
            level,
            text,
            events,
        }
    }
}

impl std::fmt::Display for SplitHeading<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Default for SplitHeading<'_> {
    fn default() -> Self {
        SplitHeading {
            level: pulldown_cmark::HeadingLevel::H1,
            text: String::new(),
            events: Vec::new(),
        }
    }
}

pub fn parse_heading(
    parser: &mut dyn Iterator<Item = pulldown_cmark::Event<'_>>,
) -> Result<(pulldown_cmark::HeadingLevel, String), String> {
    let level = match parser.next().clone() {
        Some(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level: lvl, .. })) => lvl,
        _ => return Err("Expected heading start".to_string()),
    };
    let text = match parser.next().clone() {
        Some(pulldown_cmark::Event::Text(text)) => text.to_string(),
        _ => return Err("Expected heading text".to_string()),
    };
    match parser.next().clone() {
        Some(pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(lvl))) => {
            if lvl != level {
                return Err("Heading level mismatch".to_string());
            }
        }
        _ => return Err("Expected heading end".to_string()),
    };
    Ok((level, text))
}

pub type EventChuck<'a> = Vec<pulldown_cmark::Event<'a>>;

pub trait SplitStop<State = bool>
where
    Self: Sized,
{
    fn split(&self, event: &pulldown_cmark::Event, state: &Option<State>) -> (bool, Option<State>);
    fn stop(&self, event: &pulldown_cmark::Event, state: &Option<State>) -> bool;
}

pub fn split_at_event<State>(
    events: &mut dyn Iterator<Item = pulldown_cmark::Event<'_>>,
    split_stop: impl SplitStop<State>,
) -> Vec<EventChuck<'static>> {
    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();

    let mut state = None;

    for event in events {
        if split_stop.stop(&event, &state) {
            break;
        }
        let (split, new_state) = split_stop.split(&event, &state);
        if split && !current_chunk.is_empty() {
            chunks.push(current_chunk);
            current_chunk = Vec::new();
        }
        state = new_state;
        current_chunk.push(event.into_static());
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

pub struct SplitHeadingLevel {
    pub level: pulldown_cmark::HeadingLevel,
}

impl SplitStop for SplitHeadingLevel {
    fn split(&self, event: &pulldown_cmark::Event, _state: &Option<bool>) -> (bool, Option<bool>) {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level, .. }) => {
                (level == &self.level, None)
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(_)) => (false, None),
            _ => (false, None),
        }
    }
    fn stop(&self, event: &pulldown_cmark::Event, _state: &Option<bool>) -> bool {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level, .. }) => {
                level < &self.level
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_markdown_events() {
        let text = "# Hello\n## World";
        log_markdown_events(text);
    }

    #[test]
    fn test_events_to_markdown() {
        let text = "# Hello";
        let events = pulldown_cmark::Parser::new(text).collect::<Vec<_>>();
        let result = events_to_markdown(events, None);
        assert_eq!(result, "# Hello\n");
    }

    #[test]
    fn test_split_headings() {
        let text = "# Heading 1\n## Heading A\n### Heading a\n# Heading 2\n## Heading B\n";
        let mut parser = pulldown_cmark::Parser::new(text);
        let result = split_at_event(
            &mut parser,
            SplitHeadingLevel {
                level: pulldown_cmark::HeadingLevel::H1,
            },
        );
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        let (level, text) = parse_heading(&mut result[0].clone().into_iter()).unwrap();
        assert_eq!(level, pulldown_cmark::HeadingLevel::H1);
        assert_eq!(text, "Heading 1");

        let (level, text) = parse_heading(&mut result[1].clone().into_iter()).unwrap();
        assert_eq!(level, pulldown_cmark::HeadingLevel::H1);
        assert_eq!(text, "Heading 2");
    }
}
