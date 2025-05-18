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
}

impl TryFrom<&Node> for Tag {
    type Error = &'static str;
    fn try_from(value: &Node) -> Result<Self, Self::Error> {
        match value {
            Node::Document { .. } => Ok(Self::Document),
            Node::Paragraph { .. } => Ok(Self::Paragraph),
            Node::Text(txt) => match txt {
                Text::Bold(_) => Ok(Self::Bold),
                Text::Italic(_) => Ok(Self::Italic),
                Text::Strikethrough(_) => Ok(Self::Strikethrough),
                _ => Err("Not a tag"),
            },
            Node::Heading { heading, .. } => Ok(Self::Heading(heading.level)),
            _ => todo!(),
        }
    }
}

impl Node {
    fn parse_tag<'a>(
        events: &mut impl Iterator<Item = pulldown_cmark::Event<'a>>,
        tag: &Tag,
    ) -> Result<Self, &'static str> {
        match tag {
            Tag::Italic => match events.next().ok_or("No text inside tag")? {
                pulldown_cmark::Event::Text(txt) => Ok(Self::Text(Text::Italic(txt.to_string()))),
                _ => Err("Unexpected event occurred"),
            },
            Tag::Bold => match events.next().ok_or("No text inside tag")? {
                pulldown_cmark::Event::Text(txt) => Ok(Self::Text(Text::Bold(txt.to_string()))),
                _ => Err("Unexpected event occurred"),
            },
            Tag::Strikethrough => match events.next().ok_or("No text inside tag")? {
                pulldown_cmark::Event::Text(txt) => {
                    Ok(Self::Text(Text::Strikethrough(txt.to_string())))
                }
                _ => Err("Unexpected event occurred"),
            },
            _ => todo!(),
        }
    }

    pub fn parse_nodes<'a>(
        events: &mut impl Iterator<Item = pulldown_cmark::Event<'a>>,
    ) -> Result<Vec<Self>, &'static str> {
        let mut parsing_stack = vec![];

        let mut curr_subnodes = vec![];

        while let Some(event) = events.next() {
            match event {
                pulldown_cmark::Event::Start(tag) => {
                    parsing_stack.push((
                        Self::parse_tag(events, &Tag::from_start(tag))?,
                        curr_subnodes,
                    ));
                    curr_subnodes = vec![];
                }
                pulldown_cmark::Event::Text(txt) => {
                    curr_subnodes.push(Node::Text(Text::Plain(txt.to_string())));
                }
                _ => todo!(),
            }
        }

        todo!()
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
