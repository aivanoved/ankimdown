use std::{cell::RefCell, rc::Rc, usize};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Document,
    Text(Text),
    Paragraph,
    Heading { level: usize, content: Vec<Text> },
}

#[derive(Debug, Clone)]
pub struct Node {
    node_type: NodeType,
    subnodes: Vec<Rc<RefCell<Node>>>,
}

type SharedNode = Rc<RefCell<Node>>;

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
            pulldown_cmark::Tag::Heading { level, .. } => {
                Self::Heading(level as usize)
            }
            pulldown_cmark::Tag::Paragraph => Self::Paragraph,
            _ => todo!(),
        }
    }

    fn from_end(tag_end: pulldown_cmark::TagEnd) -> Self {
        match tag_end {
            pulldown_cmark::TagEnd::Emphasis => Self::Italic,
            pulldown_cmark::TagEnd::Strong => Self::Bold,
            pulldown_cmark::TagEnd::Strikethrough => {
                Self::Strikethrough
            }
            pulldown_cmark::TagEnd::Heading(level, ..) => {
                Self::Heading(level as usize)
            }
            pulldown_cmark::TagEnd::Paragraph => Self::Paragraph,
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
                pulldown_cmark::Event::End(tag_end) => {
                    Tag::from_end(*tag_end) != tag
                }
                _ => true,
            })
            .collect::<Vec<_>>();
        let text_str = txt_events
            .iter()
            .filter_map(|event| match event {
                pulldown_cmark::Event::Text(txt) => {
                    Some(txt.to_string())
                }
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
                pulldown_cmark::Event::End(tag_end) => {
                    Tag::from_end(*tag_end) != Tag::Paragraph
                }
                _ => true,
            })
            .collect::<Vec<_>>();
        let txt_nodes =
            Self::parse_nodes(&mut text_events.into_iter())?;

        for node in &txt_nodes {
            match node.borrow().node_type {
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
            pulldown_cmark::Event::End(tag_end) => {
                Tag::from_end(*tag_end) != tag
            }
            _ => true,
        });
        let text_nodes =
            Self::parse_nodes(&mut text_nodes.into_iter())?;

        for node in &text_nodes {
            match node.borrow().node_type {
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
                level,
                content: text_nodes
                    .iter()
                    .filter_map(|node| {
                        match &node.borrow().node_type {
                            NodeType::Text(txt) => Some(txt.clone()),
                            _ => None,
                        }
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
            Tag::Italic | Tag::Bold | Tag::Strikethrough => {
                Self::parse_text_event(events, tag)
            }
            Tag::Paragraph => Self::parse_paragraph(events),
            Tag::Heading(_) => Self::parse_heading(events, tag),
        }
    }

    fn push_node(
        node: Node,
        mut nodes: Vec<SharedNode>,
        mut open_headings: Vec<SharedNode>,
    ) -> Result<(Vec<SharedNode>, Vec<SharedNode>), &'static str>
    {
        let push_level = match &node.node_type {
            NodeType::Heading { level, .. } => Some(*level),
            _ => None,
        };

        let rc_node = Rc::new(RefCell::new(node));

        if let Some(lvl) = push_level {
            while let Some(last) = open_headings.last() {
                match last.borrow().node_type {
                    NodeType::Heading { level, .. } => {
                        if level < lvl {
                            break;
                        }
                    }
                    _ => break,
                };
                open_headings.pop();
            }
        }

        if open_headings.len() == 0 {
            nodes.push(rc_node);
        } else {
            open_headings
                .last_mut()
                .ok_or("No last element")?
                .borrow_mut()
                .subnodes
                .push(rc_node);
        };

        if push_level.is_some() {
            if open_headings.len() == 0 {
                open_headings.push(
                    nodes.last().ok_or("Should have pushed")?.clone(),
                );
            } else {
                let node = open_headings
                    .last()
                    .unwrap()
                    .borrow()
                    .subnodes
                    .last()
                    .unwrap()
                    .clone();
                open_headings.push(node)
            }
        }

        Ok((nodes, open_headings))
    }

    pub fn parse_nodes(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
    ) -> Result<Vec<SharedNode>, &'static str> {
        let mut nodes = vec![];

        let mut open_headings = Vec::<SharedNode>::new();

        while let Some(event) = events.next() {
            let node = match event {
                pulldown_cmark::Event::Start(tag) => {
                    Self::parse_tag(events, Tag::from_start(tag))?
                }
                pulldown_cmark::Event::Text(txt) => Self {
                    node_type: NodeType::Text(Text::Plain(
                        txt.to_string(),
                    )),
                    subnodes: vec![],
                },
                _ => todo!(),
            };
            (nodes, open_headings) =
                Self::push_node(node, nodes, open_headings)?;
        }

        Ok(nodes)
    }

    pub fn parse_document(
        events: &mut dyn Iterator<Item = pulldown_cmark::Event>,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            node_type: NodeType::Document,
            subnodes: Self::parse_nodes(events)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::vec;

    #[test]
    fn test_parse_events() {
        let markdown = "# Heading\n\nThis is a paragraph with _italic_ and **bold** text.";
        let parser = pulldown_cmark::Parser::new(markdown);
        let events: Vec<_> = parser.collect();

        let nodes =
            Node::parse_nodes(&mut events.into_iter()).unwrap();

        assert_eq!(nodes.len(), 1);

        let heading = nodes[0].borrow();

        assert_eq!(
            heading.node_type,
            NodeType::Heading {
                level: 1,
                content: vec![Text::Plain("Heading".to_string())],
            }
        );

        assert_eq!(heading.subnodes.len(), 1);

        let paragraph = heading.subnodes.last().unwrap().borrow();

        assert_eq!(paragraph.node_type, NodeType::Paragraph);

        assert_eq!(paragraph.subnodes.len(), 5);
    }
}
