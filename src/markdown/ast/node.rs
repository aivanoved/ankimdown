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

pub enum Node {
    Document(Vec<Node>),
    Heading {
        level: usize,
        content: Vec<Text>,
        subnodes: Vec<Node>,
    },
    Text(String),
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
        fn write_indented(
            f: &mut std::fmt::Formatter<'_>,
            node: &Node,
            level: usize,
        ) -> std::fmt::Result {
            match node {
                Node::Document(subnodes) => {
                    for subnode in subnodes {
                        write_indented(f, subnode, level)?;
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
                        write_indented(f, subnode, level + 1)?;
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
                        write_indented(f, subnode, level + 1)?;
                    }
                }
                Node::List { items } => {
                    for item in items {
                        write_indented(f, item, level)?;
                    }
                }
            }
            Ok(())
        }
        write_indented(f, self, 0)
    }
}
