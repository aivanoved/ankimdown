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
    // Paragraph {
    //     subnodes: Vec<Node>,
    // },
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Node::write_indented(f, self, 0)
    }
}

impl Node {
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
            Node::Heading { heading, subnodes } => {
                write!(f, "{:indent$}{heading}", "", indent = level * 2)?;
                writeln!(f)?;
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level + 1)?;
                }
            }
            Node::Text(text) => {
                write!(f, "{:indent$}{}", "", text, indent = level * 2)?;
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
            Node::Text(text) => {
                result.push(format!("{}Text: {}", indent(level, last), text));
            }
        };
        result.join("\n")
    }

    pub fn tree_to_string(&self) -> String {
        Self::tree_view(0, self, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::markdown::text::SimpleText;

    #[test]
    fn test_write_indented() {
        let node = Node::Document {
            subnodes: vec![Node::Heading {
                heading: Heading {
                    level: 1,
                    content: vec![Text::Plain(vec![SimpleText::Simple("Heading".to_string())])],
                },
                subnodes: vec![],
            }],
        };

        let expected = "# Heading\n";

        let output = format!("{}", node);
        assert_eq!(output, expected);
    }
}
