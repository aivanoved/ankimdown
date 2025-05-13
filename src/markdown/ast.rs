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
    fn try_event_paragraph(events: &mut Iter<Event>) -> Result<Self, String> {
        let mut events_clone = events.clone();
        let mut take = 0 as usize;

        let paragraph_event = events_clone
            .next()
            .ok_or("Not paragraph start".to_string())?;

        match paragraph_event {
            Event::Start(Tag::Paragraph) => {}
            _ => return Err("Not a paragraph".to_string()),
        }

        take += 1;

        let mut text_nodes = vec![];

        while let Ok(text) = Text::try_from_events(&mut events_clone) {
            text_nodes.push(Node::Text(text));
            take += 1;
        }

        let paragraph_end = events_clone.next().ok_or("No end event")?;

        match paragraph_end {
            Event::End(TagEnd::Paragraph) => {
                let _ = events.take(take);
                Ok(Node::Paragraph {
                    subnodes: text_nodes,
                })
            }
            _ => Err("No paragraph end".to_string()),
        }
    }

    fn try_event_header(
        events: &mut Iter<Event>,
        min_level: Option<usize>,
    ) -> Result<Self, String> {
        todo!()
    }

    pub fn try_events(
        events: &mut Iter<Event>,
        min_level: Option<usize>,
    ) -> Result<Vec<Node>, String> {
        let mut nodes = vec![];
        while events.len() > 0 {
            if let Ok(paragraph) = Self::try_event_paragraph(events) {
                nodes.push(paragraph);
            } else if let Ok(heading) = Self::try_event_header(events, min_level) {
                nodes.push(heading);
            } else {
                return Err("Unsupported main node".to_string());
            }
        }

        Ok(nodes)
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
