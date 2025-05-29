use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::ankigen::db_model::table::Table;

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum CardType {
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i8)]
pub enum CardQueue {
    UserSuspended = -3,
    Buried = -2,
    Suspended = -1,
    New = 0,
    Learning = 1,
    Review = 2,
    InLearning = 3,
    Preview = 4,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum CardFlag {
    Null = 0,
    Redo = 1,
    Orange = 2,
    Green = 3,
    Blue = 4,
}

impl TryFrom<i64> for CardFlag {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = value % 8;
        match value {
            0 => Ok(CardFlag::Null),
            1 => Ok(CardFlag::Redo),
            2 => Ok(CardFlag::Orange),
            3 => Ok(CardFlag::Green),
            4 => Ok(CardFlag::Blue),
            _ => Err("Invalid CardFlag value"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Card {
    pub id: i64, // time epoch when card was created
    #[serde(rename = "nid")]
    pub note_id: usize,
    #[serde(rename = "did")]
    pub deck_id: usize,
    #[serde(rename = "ord")]
    pub ordinal: u64, // which of the card templates to use
    #[serde(rename = "mod")]
    pub modified: i64, // time epoch when card was last modified
    #[serde(rename = "usn")]
    pub update_seq_number: i64, // update sequence number
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub queue: CardQueue,
    pub due: i64, // time epoch when card is due
    #[serde(rename = "ivl")]
    pub interval: i64,
    pub factor: i64,
    #[serde(rename = "reps")]
    pub reviews: i64,
    pub lapses: i64,
    pub left: i64,
    #[serde(rename = "odue")]
    pub original_due: usize,
    #[serde(rename = "odid")]
    pub original_deck_id: usize,
    pub flags: CardFlag,
    #[serde(default)]
    pub data: String, // currently unused
}

impl Card {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        note_id: usize,
        deck_id: usize,
        ordinal: u64,
        modified: i64,
        update_seq_number: i64,
        card_type: CardType,
        queue: CardQueue,
        due: i64,
        interval: i64,
        factor: i64,
        reviews: i64,
        lapses: i64,
        left: i64,
        original_due: usize,
        original_deck_id: usize,
        flags: CardFlag,
    ) -> Self {
        Self {
            id,
            note_id,
            deck_id,
            ordinal,
            modified,
            update_seq_number,
            card_type,
            queue,
            due,
            interval,
            factor,
            reviews,
            lapses,
            left,
            original_due,
            original_deck_id,
            flags,
            data: String::new(),
        }
    }
}

impl Table for Card {
    fn table_name(&self) -> &'static str {
        "cards"
    }
}
