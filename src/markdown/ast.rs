use std::iter::Peekable;
use std::slice::Iter;

use pulldown_cmark::{Event, Tag, TagEnd};

use crate::markdown::heading::Heading;
use crate::markdown::text::Text;

#[derive(Debug, Clone)]
pub enum Node {
    Document {
        subnodes: Vec<Node>,
    },
    Text(Text),
    Heading {
        heading: Heading,
        subnodes: Vec<Node>,
    },
    Paragraph {
        subnodes: Vec<Node>,
    },
}

impl Node {
    fn parse_text(events: &mut Peekable<Iter<Event>>) -> Result<Self, &'static str> {
        let mut take = 0 as usize;
        let result = match events.peek().ok_or("No event")? {
            Event::Text(txt) => {
                take += 1;
                Ok(Node::Text(Text::Plain(vec![txt.to_string()])))
            }
            Event::HardBreak => {
                take += 1;
                Ok(Node::Text(Text::HardBreak))
            }
            Event::SoftBreak => {
                take += 1;
                Ok(Node::Text(Text::SoftBreak))
            }
            Event::Start(Tag::Emphasis) => {
                if let Event::Text(txt) = events.next().ok_or("No text")? {
                    take += 2;
                    Ok(Node::Text(Text::Italic(vec![txt.to_string()])))
                } else {
                    Err("No text")
                }
            }
            Event::Start(Tag::Strong) => {
                if let Event::Text(txt) = events.next().ok_or("No text")? {
                    take += 2;
                    Ok(Node::Text(Text::Bold(vec![txt.to_string()])))
                } else {
                    Err("No text")
                }
            }
            Event::Start(Tag::Strikethrough) => {
                if let Event::Text(txt) = events.next().ok_or("No text")? {
                    take += 2;
                    Ok(Node::Text(Text::Strikethrough(vec![txt.to_string()])))
                } else {
                    Err("No text")
                }
            }
            _ => Err("Invalid text token"),
        };
        if take > 0 {
            let _ = events.nth(take - 1);
        }
        result
    }

    fn write_indented(
        f: &mut std::fmt::Formatter<'_>,
        node: &Node,
        level: usize,
    ) -> std::fmt::Result {
        match node {
            Node::Document { subnodes } => {
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level)?;
                }
            }
            Node::Text(text) => {
                write!(f, "{:indent$}{}", "", text, indent = level * 2)?;
            }
            Node::Heading { heading, subnodes } => {
                write!(f, "{:indent$}{heading}", "", indent = level * 2)?;
                writeln!(f)?;
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level + 1)?;
                }
            }
            Node::Paragraph { subnodes } => {
                for node in subnodes {
                    Self::write_indented(f, node, level)?;
                }
            }
        }
        Ok(())
    }

    fn tree_view(level: usize, node: &Node, last: bool) -> String {
        let mut result = Vec::new();
        fn indent(level: usize, last: bool) -> String {
            let branch = if last { "└─" } else { "├─" };
            match level {
                0 => "".to_string(),
                1 => branch.to_string(),
                _ => "  ".repeat(level - 1) + branch,
            }
        }
        match node {
            Node::Document { subnodes } => {
                result.push(format!("{}Document", indent(level, last)));
                let size = subnodes.len();
                for (idx, node) in subnodes.iter().enumerate() {
                    result.push(Self::tree_view(level + 1, node, idx == size - 1));
                }
            }
            Node::Text(text) => {
                result.push(format!("{}Text: {}", indent(level, last), text));
            }
            Node::Heading { heading, subnodes } => {
                result.push(format!(
                    "{}Heading: #H{}",
                    indent(level, last),
                    heading.level,
                ));
                result.push(format!(
                    "{}Text: {}",
                    indent(level + 1, subnodes.is_empty()),
                    heading
                        .content
                        .iter()
                        .map(|text| text.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ));
                let size = subnodes.len();
                for (idx, subnode) in subnodes.iter().enumerate() {
                    result.push(Self::tree_view(level + 1, subnode, idx == size - 1));
                }
            }
            Node::Paragraph { subnodes } => {
                result.push(format!("{}Paragraph", indent(level, last)));
                let size = subnodes.len();
                for (idx, subnode) in subnodes.iter().enumerate() {
                    result.push(Self::tree_view(level + 1, subnode, idx == size - 1));
                }
            }
        };
        result.join("\n")
    }

    pub fn tree_to_string(&self) -> String {
        Self::tree_view(0, self, true)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Node::write_indented(f, self, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_indented() {
        let node = Node::Document {
            subnodes: vec![Node::Heading {
                heading: Heading {
                    level: 1,
                    content: vec![Text::Plain(vec!["Heading".to_string()])],
                },
                subnodes: vec![],
            }],
        };

        let expected = "# Heading\n";

        let output = format!("{}", node);
        assert_eq!(output, expected);
    }
}
