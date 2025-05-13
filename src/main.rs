use ankimdown::markdown::util::log_markdown_str;

#[allow(dead_code)]
fn markdown_ast() {
    let markdown_text: &str = r#"abc _dfg_ **hij** ~~jkl~~"#;

    log_markdown_str(markdown_text);
}

#[allow(dead_code)]
fn markdown_parser() {
    let markdown_text: &str = r#"
# Deck name

# hello
## Meaning
a **greeting**
- 1
- 2
- 3
"#;

    log_markdown_str(markdown_text);
}

fn main() {
    markdown_ast();
    // markdown_parser();
}
