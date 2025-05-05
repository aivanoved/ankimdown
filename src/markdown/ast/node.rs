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

    fn tree_view(level: usize, node: &Node) -> String {
        let mut result = Vec::new();
        fn indent(level: usize) -> String {
            match level {
                0 => "".to_string(),
                1 => "├─".to_string(),
                _ => "│ ".repeat(level - 1) + "├─",
            }
        }
        match node {
            Node::Document(nodes) => {
                if level > 0 {
                    result.push(format!("{}Document", indent(level)));
                } else {
                    result.push("Document".to_string());
                }
                for node in nodes {
                    result.push(Self::tree_view(level + 1, node));
                }
            }
            Node::Heading {
                level: h_level,
                content,
                subnodes,
            } => {
                result.push("│ ".repeat(level).to_string());
                result.push(format!("{}Heading: #H{}", indent(level), *h_level,));
                for text in content {
                    result.push(format!("{}Text: {}", indent(level + 1), text));
                }
                for subnode in subnodes {
                    result.push(Self::tree_view(level + 1, subnode));
                }
            }
            Node::Text(text) => {
                result.push(format!("{}Text: {}", indent(level), text));
            }
            Node::ListItem {
                text,
                order,
                subnodes,
            } => {
                match order {
                    ListOrderType::Unordered => {
                        result.push(format!("{}ListItem: Unordered", indent(level)));
                    }
                    ListOrderType::Ordered(num) => {
                        result.push(format!("{}ListItem: Ordered {}", indent(level), num));
                    }
                }
                result.push(format!(
                    "{}Text: {}",
                    indent(level + 1),
                    text.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ));
                for node in subnodes {
                    result.push(Self::tree_view(level + 1, node));
                }
            }
            Node::List { items } => {
                for item in items {
                    result.push(Self::tree_view(level, item));
                }
            }
        };
        result.join("\n")
    }

    pub fn tree_to_string(&self) -> String {
        Self::tree_view(0, self)
    }
}
