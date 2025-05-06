use ankimdown::markdown::ast::*;

// const SAMPLE_MARKDOWN_DECK: &str = r#"
// # Deck name
//
// ## Deck metadata:
//
// - Maps:
//     - Description: Meaning
//
// # hello
// ## Meaning
// 1. a greeting
//
// ## Metadata
//
// - Templates:
//     - Simple
//     - Reverse
// - Autogen:
//     - id: 0
// "#;

fn main() {
    let sample_markdown_ast: Node = Node::Document(vec![Node::Heading {
        level: 1,
        content: vec![Text::Plain("Deck name".to_string())],
        subnodes: vec![Node::Heading {
            level: 2,
            content: vec![Text::Plain("Deck metadata:".to_string())],
            subnodes: vec![Node::List {
                items: vec![Node::ListItem {
                    text: vec![Text::Plain("Maps:".to_string())],
                    order: ListOrderType::Unordered,
                    subnodes: vec![],
                }],
            }],
        }],
    }]);

    println!("{}", sample_markdown_ast.tree_to_string());
    println!("{:#?}", sample_markdown_ast)
}
