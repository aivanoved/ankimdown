use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn log_markdown_events(events: &mut dyn Iterator<Item = Event>) {
    let mut width = 0;
    eprintln!("[");
    for event in events {
        if let Event::End(_) = event {
            width -= 2;
        }
        eprintln!("    {:width$}{event:?}", "");
        if let Event::Start(_) = event {
            width += 2;
        }
    }
    eprintln!("]");
}

pub fn log_markdown_str(text: &str) {
    eprintln!("{text:?}:");
    log_markdown_events(&mut Parser::new_ext(text, Options::all()));
}

pub fn check_matching_tags(tag: &Tag, tag_end: &TagEnd) -> bool {
    let expected_end = match tag {
        Tag::Emphasis => TagEnd::Emphasis,
        Tag::Strikethrough => TagEnd::Strikethrough,
        Tag::Strong => TagEnd::Strong,
        _ => todo!(),
    };

    expected_end == tag_end.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_markdown_events() {
        let text = "# Hello\n## World";
        log_markdown_str(text);
    }
}
