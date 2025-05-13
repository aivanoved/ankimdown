#[derive(Debug, Clone)]
pub enum Text {
    Plain(String),
    Bold(String),
    Italic(String),
    Underline(String),
    Strikethrough(String),
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text::Plain(text) => write!(f, "{}", text),
            Text::Bold(text) => write!(f, "**{}**", text),
            Text::Italic(text) => write!(f, "*{}*", text),
            Text::Underline(text) => write!(f, "__{}__", text),
            Text::Strikethrough(text) => write!(f, "~~{}~~", text),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ListOrderType {
    Unordered,
    Ordered(usize),
}

impl std::fmt::Display for ListOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListOrderType::Unordered => write!(f, "-"),
            ListOrderType::Ordered(num) => write!(f, "{}.", num),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Document(Vec<Node>),
    Heading {
        level: usize,
        content: Vec<Text>,
        subnodes: Vec<Node>,
    },
    Text(Text),
    ListItem {
        text: Vec<Text>,
        order: ListOrderType,
        subnodes: Vec<Node>,
    },
    List {
        items: Vec<Node>,
    },
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
            Node::Document(subnodes) => {
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level)?;
                }
            }
            Node::Heading {
                level: h_level,
                content,
                subnodes,
            } => {
                write!(f, "{:indent$}{} ", "", "#".repeat(*h_level), indent = level)?;
                for text in content {
                    write!(f, "{}", text)?;
                }
                writeln!(f)?;
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level + 1)?;
                }
            }
            Node::Text(text) => {
                write!(f, "{:indent$}{}", "", text, indent = level * 2)?;
            }
            Node::ListItem {
                text,
                order,
                subnodes,
            } => {
                write!(f, "{:indent$}{} ", "", order, indent = level * 2)?;
                for t in text {
                    write!(f, "{}", t)?;
                }
                writeln!(f)?;
                for subnode in subnodes {
                    Self::write_indented(f, subnode, level + 1)?;
                }
            }
            Node::List { items } => {
                for item in items {
                    Self::write_indented(f, item, level)?;
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
            Node::Document(nodes) => {
                result.push(format!("{}Document", indent(level, last)));
                let size = nodes.len();
                for (idx, node) in nodes.iter().enumerate() {
                    result.push(Self::tree_view(level + 1, node, idx == size - 1));
                }
            }
            Node::Heading {
                level: h_level,
                content,
                subnodes,
            } => {
                result.push(format!("{}Heading: #H{}", indent(level, last), *h_level,));
                result.push(format!(
                    "{}Text: {}",
                    indent(level + 1, subnodes.is_empty()),
                    content
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
            Node::ListItem {
                text,
                order,
                subnodes,
            } => {
                match order {
                    ListOrderType::Unordered => {
                        result.push(format!("{}ListItem: Unordered", indent(level, last)));
                    }
                    ListOrderType::Ordered(num) => {
                        result.push(format!("{}ListItem: Ordered {}", indent(level, last), num));
                    }
                }
                result.push(format!(
                    "{}Text: {}",
                    indent(level + 1, subnodes.is_empty()),
                    text.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ));
                let size = subnodes.len();
                for (idx, node) in subnodes.iter().enumerate() {
                    result.push(Self::tree_view(level + 1, node, idx == size - 1));
                }
            }
            Node::List { items } => {
                let size = items.len();
                for (idx, item) in items.iter().enumerate() {
                    result.push(Self::tree_view(level, item, idx == size - 1));
                }
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

    #[test]
    fn test_write_indented() {
        let node = Node::Document(vec![
            Node::Heading {
                level: 1,
                content: vec![Text::Plain("Heading".to_string())],
                subnodes: vec![],
            },
            Node::ListItem {
                text: vec![Text::Plain("Item 1".to_string())],
                order: ListOrderType::Unordered,
                subnodes: vec![],
            },
            Node::ListItem {
                text: vec![Text::Plain("Item 2".to_string())],
                order: ListOrderType::Ordered(1),
                subnodes: vec![],
            },
        ]);

        let expected = "# Heading\n\
                       - Item 1\n\
                       1. Item 2\n";

        let output = format!("{}", node);
        assert_eq!(output, expected);
    }
}
