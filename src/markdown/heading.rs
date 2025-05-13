use std::slice::Iter;

use pulldown_cmark::{Event, Tag, TagEnd};

use crate::markdown::text::Text;

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: usize,
    pub content: Vec<Text>,
}

impl std::fmt::Display for Heading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", "#".repeat(self.level))?;
        for text in self.content.clone() {
            write!(f, "{}", text)?;
        }
        Ok(())
    }
}

impl Heading {
    pub fn try_from_events(events: &mut Iter<Event>) -> Result<Self, String> {
        let mut events_cloned = events.clone();

        let heading_event = events_cloned.next().ok_or("No heading start event")?;

        let mut heading = match heading_event {
            Event::Start(Tag::Heading { level, .. }) => Heading {
                level: *level as usize,
                content: vec![],
            },
            _ => return Err("Not a heading tag".to_string()),
        };

        heading
            .content
            .push(Text::try_from_events(&mut events_cloned)?);

        match events_cloned.next().ok_or("No heading closing event")? {
            Event::End(TagEnd::Heading(level)) => {
                if *level as usize != heading.level {
                    Err("Heading closing does not match opening tag level".to_string())
                } else {
                    Ok(heading)
                }
            }
            _ => Err("No heading closing tag".to_string()),
        }
    }
}
