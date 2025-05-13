use pulldown_cmark;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_markdown_events() {
        let text = "# Hello\n## World";
        log_markdown_events(text);
    }
}
