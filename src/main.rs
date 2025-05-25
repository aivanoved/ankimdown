use ankimdown::markdown::ast::Node;
use ankimdown::markdown::util::log_markdown_str;
use pulldown_cmark::Parser;

#[allow(dead_code)]
fn markdown_ast() {
    let markdown_text: &str = r#"abc _dfg_ **hij** ~~jkl~~"#;

    log_markdown_str(markdown_text);
}

#[allow(dead_code)]
fn markdown_parser() {
    let markdown_text: &str = r#"
# Deck __name__

# hello
## Meaning
a **greeting**
"#;

    log_markdown_str(markdown_text);
    let nodes =
        Node::parse_nodes(&mut Parser::new(markdown_text)).unwrap();
    println!("{:#?}", nodes);
}

fn main() {
    // markdown_ast();
    markdown_parser();
}
