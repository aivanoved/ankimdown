use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::table::Table;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Deck {
    pub name: String,
    #[serde(rename = "extendRev")]
    pub extended_review_limit: Option<usize>,
    #[serde(rename = "usn")]
    pub update_seq_number: i64,
    pub collapsed: bool,
    #[serde(rename = "browserCollapsed")]
    pub browser_collapsed: bool,
    #[serde(rename = "newToday")]
    pub new_today: [usize; 2],
    #[serde(rename = "revToday")]
    pub review_today: [usize; 2],
    #[serde(rename = "lrnToday")]
    pub learn_today: [usize; 2],
    // Unused in the code
    #[serde(rename = "timeToday")]
    pub time_today: [usize; 2],
    #[serde(rename = "dyn")]
    pub filtered: usize,
    #[serde(rename = "extendNew")]
    pub extended_new_limit: Option<usize>,
    #[serde(rename = "conf")]
    pub config_id: Option<usize>,
    pub id: i64,
    #[serde(rename = "mod")]
    pub modified: i64,
    #[serde(rename = "desc")]
    pub description: String,
}

impl Deck {
    pub fn new(name: String) -> Self {
        Self {
            name,
            extended_review_limit: None,
            update_seq_number: 0,
            collapsed: false,
            browser_collapsed: false,
            new_today: [0, 0],
            review_today: [0, 0],
            learn_today: [0, 0],
            time_today: [0, 0],
            filtered: 0,
            extended_new_limit: None,
            config_id: None,
            id: 0,
            modified: 0,
            description: String::new(),
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new("default".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LapseConfig {
    pub delays: Vec<usize>,
    #[serde(rename = "leechAction")]
    pub leech_action: usize,
    #[serde(rename = "leechFails")]
    pub leech_fails: usize,
    #[serde(rename = "minInt")]
    pub min_interval: usize,
    #[serde(rename = "mult")]
    pub interval_increase: usize,
}

impl LapseConfig {
    pub fn new() -> Self {
        Self {
            delays: vec![0],
            leech_action: 0,
            leech_fails: 0,
            min_interval: 0,
            interval_increase: 0,
        }
    }
}

impl Default for LapseConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum NewCardOrder {
    Random = 0,
    Due = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewConfig {
    pub bury: bool,
    pub delays: Vec<usize>,
    #[serde(rename = "initialFactor")]
    pub initial_factor: usize,
    #[serde(rename = "ints")]
    pub intervals: Vec<usize>,
    pub order: NewCardOrder,
    #[serde(rename = "perDay")]
    pub per_day: usize,
    pub separate: bool,
}

impl NewConfig {
    pub fn new() -> Self {
        Self {
            bury: false,
            delays: vec![0],
            initial_factor: 0,
            intervals: vec![0],
            order: NewCardOrder::Random,
            per_day: 0,
            separate: false,
        }
    }
}

impl Default for NewConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReviewConfig {
    pub bury: bool,
    #[serde(rename = "ease4")]
    pub ease_factor: usize,
    pub fuzz: usize,
    #[serde(rename = "ivlFct")]
    pub interval_factor: usize,
    #[serde(rename = "maxIvl")]
    pub max_interval: usize,
    // Unused
    #[serde(rename = "minSpace")]
    pub min_space: usize,
    #[serde(rename = "perDay")]
    pub cards_daily: usize,
}

impl ReviewConfig {
    pub fn new() -> Self {
        Self {
            bury: false,
            ease_factor: 0,
            fuzz: 0,
            interval_factor: 0,
            max_interval: 0,
            min_space: 0,
            cards_daily: 0,
        }
    }
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Table for Deck {
    fn table_name(&self) -> &'static str {
        "decks"
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeckConfig {
    pub autoplay: bool,
    #[serde(rename = "dyn")]
    pub filtered: Option<bool>,
    pub id: Option<usize>,
    #[serde(rename = "lapse")]
    pub lapse_config: LapseConfig,
    #[serde(rename = "maxTaken")]
    pub max_taken: usize,
    #[serde(rename = "mod")]
    pub modified: usize,
    pub name: String,
    #[serde(rename = "new")]
    pub new_config: NewConfig,
    #[serde(rename = "replayq")]
    pub replay_question: bool,
    #[serde(rename = "rev")]
    pub review_config: ReviewConfig,
    pub timer: bool,
    #[serde(rename = "usn")]
    pub update_seq_number: usize,
}

impl DeckConfig {
    pub fn new(name: String) -> Self {
        Self {
            autoplay: false,
            filtered: None,
            id: None,
            lapse_config: LapseConfig::new(),
            max_taken: 0,
            modified: 0,
            name,
            new_config: NewConfig::new(),
            replay_question: false,
            review_config: ReviewConfig::new(),
            timer: false,
            update_seq_number: 0,
        }
    }
}
