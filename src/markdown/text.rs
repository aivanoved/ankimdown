use pulldown_cmark::{Event, Tag};
use std::slice::Iter;

use crate::markdown::util::check_matching_tags;

#[derive(Debug, Clone)]
pub enum SimpleText {
    Simple(String),
    SoftBreak,
    HardBreak,
}

impl std::fmt::Display for SimpleText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(txt) => write!(f, "{}", txt),
            Self::SoftBreak => write!(f, "\n"),
            Self::HardBreak => write!(f, "\\\n"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Text {
    Plain(Vec<SimpleText>),
    Bold(Vec<SimpleText>),
    Italic(Vec<SimpleText>),
    Strikethrough(Vec<SimpleText>),
}

impl Text {
    pub fn try_from_events(events: &mut Iter<Event>) -> Result<Self, String> {
        let mut take: usize = 0;

        let mut events_cloned = events.clone();

        let event = events_cloned
            .next()
            .ok_or("Expected nonempty iterator".to_string())?;

        take += 1;

        let tag = match event {
            Event::Text(txt) => {
                let _ = events.nth(take - 1);
                return Ok(Self::Plain(vec![SimpleText::Simple(txt.to_string())]));
            }
            Event::HardBreak => {
                let _ = events.nth(take - 1);
                return Ok(Text::Plain(vec![SimpleText::HardBreak]));
            }
            Event::SoftBreak => {
                let _ = events.nth(take - 1);
                return Ok(Text::Plain(vec![SimpleText::SoftBreak]));
            }
            Event::Start(tag) => tag,
            _ => return Err("Unable to parse".to_string()),
        };

        match tag {
            Tag::Emphasis | Tag::Strong | Tag::Strikethrough => {}
            _ => return Err("Unexpected tag".to_string()),
        };

        let mut closed = false;

        let mut inner_text = vec![];

        while let Some(event) = events_cloned.next() {
            match event {
                Event::Text(txt) => {
                    inner_text.push(SimpleText::Simple(txt.to_string()));
                }
                Event::HardBreak => inner_text.push(SimpleText::HardBreak),
                Event::SoftBreak => inner_text.push(SimpleText::SoftBreak),
                Event::End(tag_end) => {
                    if !check_matching_tags(&tag, &tag_end) {
                        return Err("Tags are not matching".to_string());
                    } else {
                        closed = true
                    }
                    take += 1;
                    break;
                }
                _ => return Err("Unable to parse again".to_string()),
            };
            take += 1;
        }

        if !closed {
            return Err("Tag left unclosed".to_string());
        }

        match tag {
            Tag::Emphasis => {
                let _ = events.nth(take - 1);
                Ok(Text::Italic(inner_text))
            }
            Tag::Strikethrough => {
                let _ = events.nth(take - 1);
                Ok(Text::Strikethrough(inner_text))
            }
            Tag::Strong => {
                let _ = events.nth(take - 1);
                Ok(Text::Bold(inner_text))
            }
            _ => Err("Invalid text tag".to_string()),
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (surround, text) = match self {
            Text::Plain(txt) => (None, txt),
            Text::Bold(txt) => (Some("**"), txt),
            Text::Italic(txt) => (Some("_"), txt),
            Text::Strikethrough(txt) => (Some("~~"), txt),
        };

        let inner_text = text
            .iter()
            .map(|txt| format!("{}", txt))
            .collect::<Vec<String>>()
            .join("");

        let sep = surround.unwrap_or("");

        write!(f, "{sep}{inner_text}{sep}")
    }
}
