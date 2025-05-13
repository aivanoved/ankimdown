use ankimdown::markdown::{
    text::Text,
    util::{log_markdown_events, log_markdown_str},
};
use pulldown_cmark::{Event, Options, Parser};

#[allow(dead_code)]
fn markdown_ast() {
    let markdown_text: &str = r#"abc _dfg_ **hij** ~~jkl~~"#;

    log_markdown_str(markdown_text);

    let mut events = Parser::new_ext(markdown_text, Options::all()).collect::<Vec<Event>>();
    events = events[1..events.len() - 1].to_vec();

    log_markdown_events(&mut events.clone().into_iter());

    let mut events_iter = events.iter();

    for _ in 0..6 {
        println!(
            "{}",
            format!("{}", Text::try_from_events(&mut events_iter).unwrap())
        )
    }
}

#[allow(dead_code)]
fn markdown_parser() {
    let markdown_text: &str = r#"
# Deck name

# hello
## Meaning
a **greeting**
"#;

    log_markdown_str(markdown_text);
}

fn main() {
    // markdown_ast();
    markdown_parser();
}
