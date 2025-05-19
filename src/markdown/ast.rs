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
            Self::Italic(txt) => {
                let separator = "_";
                format!("{separator}{}{separator}", txt)
            }
            Self::Bold(txt) => {
                let separator = "**";
                format!("{separator}{}{separator}", txt)
            }
            Self::Strikethrough(txt) => {
                let separator = "~~";
                format!("{separator}{}{separator}", txt)
            }
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
            pulldown_cmark::TagEnd::Emphasis => Self::Italic,
            pulldown_cmark::TagEnd::Strong => Self::Bold,
            pulldown_cmark::TagEnd::Strikethrough => Self::Strikethrough,
            _ => todo!(),
        }
    }
}

impl Node {
    fn parse_text_event(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
        tag: Tag,
    ) -> Result<Self, &'static str> {
        let txt_events = events
            .take_while(|event| match event {
                pulldown_cmark::Event::End(tag_end) => Tag::from_end(*tag_end) != tag,
                _ => true,
            })
            .collect::<Vec<_>>();
        let text_str = txt_events
            .iter()
            .filter_map(|event| match event {
                pulldown_cmark::Event::Text(txt) => Some(txt.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();
        if txt_events.len() != text_str.len() {
            return Err("Not all events are text events");
        }
        let text = text_str.join("");
        match tag {
            Tag::Italic => Ok(Self {
                node_type: NodeType::Text(Text::Italic(text)),
                subnodes: vec![],
            }),
            Tag::Bold => Ok(Self {
                node_type: NodeType::Text(Text::Bold(text)),
                subnodes: vec![],
            }),
            Tag::Strikethrough => Ok(Self {
                node_type: NodeType::Text(Text::Strikethrough(text)),
                subnodes: vec![],
            }),
            _ => Err("Not a text event"),
        }
    }

    fn parse_paragraph(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
    ) -> Result<Self, &'static str> {
        let text_events = events
            .take_while(|event| match event {
                pulldown_cmark::Event::End(tag_end) => Tag::from_end(*tag_end) != Tag::Paragraph,
                _ => true,
            })
            .collect::<Vec<_>>();
        let txt_nodes = Self::parse_nodes(&mut text_events.into_iter())?;

        for node in &txt_nodes {
            match node.node_type {
                NodeType::Text(_) => (),
                _ => return Err("Non text node was found"),
            }
        }

        Ok(Self {
            node_type: NodeType::Paragraph,
            subnodes: txt_nodes,
        })
    }

    fn parse_heading(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
        tag: Tag,
    ) -> Result<Self, &'static str> {
        let text_nodes = events.take_while(|event| match event {
            pulldown_cmark::Event::End(tag_end) => Tag::from_end(*tag_end) != tag,
            _ => true,
        });
        let text_nodes = Self::parse_nodes(&mut text_nodes.into_iter())?;

        for node in &text_nodes {
            match node.node_type {
                NodeType::Text(_) => (),
                _ => return Err("Non text node was found"),
            }
        }

        let level = match tag {
            Tag::Heading(level) => Some(level),
            _ => None,
        }
        .ok_or("Not headinh tag")?;

        Ok(Self {
            node_type: NodeType::Heading {
                level: level,
                content: text_nodes
                    .iter()
                    .filter_map(|node| match &node.node_type {
                        NodeType::Text(txt) => Some(txt.clone()),
                        _ => None,
                    })
                    .collect(),
            },
            subnodes: vec![],
        })
    }

    fn parse_tag(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
        tag: Tag,
    ) -> Result<Self, &'static str> {
        match tag {
            Tag::Italic | Tag::Bold | Tag::Strikethrough => Self::parse_text_event(events, tag),
            Tag::Paragraph => Self::parse_paragraph(events),
            Tag::Heading(_) => Self::parse_heading(events, tag),
        }
    }

    pub fn parse_nodes(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
    ) -> Result<Vec<Self>, &'static str> {
        let mut nodes = vec![];

        let mut open_headings = Vec::<usize>::new();
        let mut subnodes_stack = vec![&mut nodes];

        while let Some(event) = events.next() {
            let mut node = match event {
                pulldown_cmark::Event::Start(tag) => Self::parse_tag(events, Tag::from_start(tag))?,
                pulldown_cmark::Event::Text(txt) => Self {
                    node_type: NodeType::Text(Text::Plain(txt.to_string())),
                    subnodes: vec![],
                },
                _ => todo!(),
            };

            if let NodeType::Heading { level: lvl, .. } = node.node_type.clone() {
                if open_headings.iter().any(|heading| lvl <= *heading) {
                    loop {
                        todo!();
                    }
                } else {
                    let size = subnodes_stack.len();
                    subnodes_stack[size - 1].push(node);
                    open_headings.push(lvl);
                }
            } else {
                let size = subnodes_stack.len();
                subnodes_stack[size - 1].push(node);
            }
        }

        Ok(nodes)
    }

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
