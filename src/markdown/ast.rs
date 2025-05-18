#[derive(Debug, Clone)]
pub enum Text {
    Plain(String),
    Italic(String),
    Bold(String),
    Strikethrough(String),
    SoftBrake,
    HardBrake,
}

impl Text {
    pub fn to_markdown(&self) -> String {
        match self {
            Self::Plain(txt) => txt.to_string(),
            Self::Italic(txt) => txt.to_string(),
            Self::Bold(txt) => txt.to_string(),
            Self::Strikethrough(txt) => txt.to_string(),
            Self::SoftBrake => "\n".to_string(),
            Self::HardBrake => "\\\n".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Document,
    Text(Text),
    Paragraph,
    Heading { level: usize, content: Vec<Text> },
}

#[derive(Debug, Clone)]
pub struct Node {
    node_type: NodeType,
    subnodes: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tag {
    Document,
    Italic,
    Bold,
    Strikethrough,
    Paragraph,
    Heading(usize),
}

impl Tag {
    fn from_start(tag: pulldown_cmark::Tag) -> Self {
        match tag {
            pulldown_cmark::Tag::Emphasis => Self::Italic,
            pulldown_cmark::Tag::Strong => Self::Bold,
            pulldown_cmark::Tag::Strikethrough => Self::Strikethrough,
            pulldown_cmark::Tag::Heading { level, .. } => Self::Heading(level as usize),
            pulldown_cmark::Tag::Paragraph => Self::Paragraph,
            _ => todo!(),
        }
    }

    fn from_end(tag_end: pulldown_cmark::TagEnd) -> Self {
        match tag_end {
            _ => todo!(),
        }
    }
}

impl Node {
    fn write_indented(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Document => {
                for node in &self.subnodes {
                    node.write_indented(f, level)?;
                }
            }
            NodeType::Text(txt) => {
                write!(f, "{:indent$}{}", "", txt.to_markdown(), indent = level * 2)?;
            }
            NodeType::Heading { level, content } => {
                write!(
                    f,
                    "{:indent$}{}",
                    "",
                    content
                        .iter()
                        .map(|txt| txt.to_markdown())
                        .collect::<Vec<_>>()
                        .join(""),
                    indent = level * 2
                )?;
                writeln!(f)?;
                for node in &self.subnodes {
                    node.write_indented(f, level + 1)?;
                }
            }
            NodeType::Paragraph => {
                for node in &self.subnodes {
                    node.write_indented(f, level)?;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_indented(f, 0)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_write_indented() {
        // let node = Node::Document {
        //     subnodes: vec![Node::Heading {
        //         heading: Heading {
        //             level: 1,
        //             content: vec![Text::Plain("Heading".to_string())],
        //         },
        //         subnodes: vec![],
        //     }],
        // };

        let node = Node {
            node_type: NodeType::Document,
            subnodes: vec![Node {
                node_type: NodeType::Heading {
                    level: 1,
                    content: vec![Text::Plain("Heading".to_string())],
                },
                subnodes: vec![],
            }],
        };

        let expected = "# Heading\n";

        let output = format!("{}", node);
        assert_eq!(output, expected);
    }
}
