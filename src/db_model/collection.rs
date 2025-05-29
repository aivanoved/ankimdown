use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::db_model::deck::{Deck, DeckConfig};
use crate::db_model::model::Model;

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq,
)]
#[repr(u8)]
pub enum NewSpread {
    NewCardsDistribute = 0,
    NewCardsLast = 1,
    NewCardsFirst = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollectionConfig {
    #[serde(rename = "curDeck")]
    pub current_deck: i64,
    #[serde(rename = "activeDecks")]
    pub active_decks: Vec<i64>,
    #[serde(rename = "newSpread")]
    pub new_spread: NewSpread,
    #[serde(rename = "collapseTime")]
    pub collapse_time: i64,
    #[serde(rename = "timeLim")]
    pub time_limit: i64,
    #[serde(rename = "estTimes")]
    pub estimated_times: bool,
    #[serde(rename = "dueCounts")]
    pub due_counts: bool,
    #[serde(rename = "curModel")]
    pub current_model: String,
    #[serde(rename = "nextPos")]
    pub next_position: i64,
    #[serde(rename = "sortType")]
    pub sort_type: String,
    #[serde(rename = "sortBackwards")]
    pub sort_backwards: bool,
    #[serde(rename = "addToCur")]
    pub add_to_current: bool,
    #[serde(rename = "dayLearnFirst")]
    pub day_learn_first: bool,
    #[serde(rename = "newBury")]
    pub new_bury: bool,
    #[serde(rename = "lastUnburied")]
    pub last_unburied: Option<i64>,
    #[serde(rename = "activeCols")]
    pub active_columns: Option<Vec<String>>,
}

impl CollectionConfig {
    pub fn new() -> Self {
        Self {
            current_deck: 0,
            active_decks: vec![],
            new_spread: NewSpread::NewCardsDistribute,
            collapse_time: 0,
            time_limit: 0,
            estimated_times: false,
            due_counts: false,
            current_model: String::new(),
            next_position: 0,
            sort_type: String::new(),
            sort_backwards: false,
            add_to_current: false,
            day_learn_first: false,
            new_bury: true,
            last_unburied: None,
            active_columns: None,
        }
    }
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Collection {
    // Any number, there is only one collection
    pub id: i64,
    #[serde(rename = "crt")]
    pub created: i64,
    // Last modified time
    #[serde(rename = "mod")]
    pub modified: i64,
    #[serde(rename = "scm")]
    // Last modified time of the scheme
    // If the scm is different from the scheme a sync is required
    pub scheme_mod_time: i64,
    #[serde(rename = "ver")]
    pub version: usize,
    // Unused, set to 0
    #[serde(rename = "dty")]
    pub dirty: i64,
    // Update seq number
    #[serde(rename = "usn")]
    pub update_seq_number: i64,
    // Last sync time
    #[serde(rename = "lsn")]
    pub last_sync_time: i64,
    // JSON configuration object
    #[serde(rename = "conf")]
    pub config: CollectionConfig,
    // JSON object of the model
    // keys are string of integers, creation time of the model
    // values are JSON object of the model
    #[serde(
        serialize_with = "Collection::serialize_models",
        deserialize_with = "Collection::deserialize_models"
    )]
    pub models: Vec<(usize, Model)>,
    // JSON object of the decks
    // keys are string of integers, creation time of the deck
    // values are JSON object of the deck
    #[serde(
        serialize_with = "Collection::serialize_decks",
        deserialize_with = "Collection::deserialize_decks"
    )]
    pub decks: Vec<(usize, Deck)>,
    // JSON object for the configuration for decks
    // keys are strings, configuration creation time
    // values are JSON object of the configuration
    #[serde(
        rename = "dconf",
        serialize_with = "Collection::serialize_deck_configs",
        deserialize_with = "Collection::deserialize_deck_configs"
    )]
    pub deck_configs: Vec<(usize, DeckConfig)>,
    pub tags: String,
}

impl Collection {
    fn serialize_models<S>(
        models: &[(usize, Model)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut result =
            serializer.serialize_map(Some(models.len()))?;
        for (model_time, model) in models {
            let model_json = serde_json::to_value(model)
                .map_err(serde::ser::Error::custom)?;
            result.serialize_entry(
                &model_time.to_string(),
                &model_json.to_string(),
            )?;
        }
        result.end()
    }

    fn deserialize_models<'de, D>(
        deserializer: D,
    ) -> Result<Vec<(usize, Model)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CollectionVisitor;
        impl<'de> Visitor<'de> for CollectionVisitor {
            type Value = Vec<(usize, Model)>;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a map of models")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut models = Vec::new();
                while let Some((key, value)) =
                    map.next_entry::<String, String>()?
                {
                    let model_time = key
                        .parse::<usize>()
                        .map_err(de::Error::custom)?;
                    let model: Model = serde_json::from_str(&value)
                        .map_err(de::Error::custom)?;
                    models.push((model_time, model));
                }
                Ok(models)
            }
        }
        deserializer.deserialize_map(CollectionVisitor)
    }

    fn serialize_decks<S>(
        decks: &[(usize, Deck)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut result =
            serializer.serialize_map(Some(decks.len()))?;
        for (deck_time, deck) in decks {
            let deck_json = serde_json::to_value(deck)
                .map_err(serde::ser::Error::custom)?;
            result.serialize_entry(
                &deck_time.to_string(),
                &deck_json.to_string(),
            )?;
        }
        result.end()
    }

    fn deserialize_decks<'de, D>(
        deserializer: D,
    ) -> Result<Vec<(usize, Deck)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CollectionVisitor;
        impl<'de> Visitor<'de> for CollectionVisitor {
            type Value = Vec<(usize, Deck)>;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a map of decks")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut decks = Vec::new();
                while let Some((key, value)) =
                    map.next_entry::<String, String>()?
                {
                    let deck_time = key
                        .parse::<usize>()
                        .map_err(de::Error::custom)?;
                    let deck: Deck = serde_json::from_str(&value)
                        .map_err(de::Error::custom)?;
                    decks.push((deck_time, deck));
                }
                Ok(decks)
            }
        }
        deserializer.deserialize_map(CollectionVisitor)
    }

    fn serialize_deck_configs<S>(
        deck_configs: &[(usize, DeckConfig)],
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut result =
            serializer.serialize_map(Some(deck_configs.len()))?;
        for (deck_time, deck_config) in deck_configs {
            let deck_config_json = serde_json::to_value(deck_config)
                .map_err(serde::ser::Error::custom)?;
            result.serialize_entry(
                &deck_time.to_string(),
                &deck_config_json.to_string(),
            )?;
        }
        result.end()
    }

    fn deserialize_deck_configs<'de, D>(
        deserializer: D,
    ) -> Result<Vec<(usize, DeckConfig)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CollectionVisitor;
        impl<'de> Visitor<'de> for CollectionVisitor {
            type Value = Vec<(usize, DeckConfig)>;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("a map of deck configs")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut deck_configs = Vec::new();
                while let Some((key, value)) =
                    map.next_entry::<String, String>()?
                {
                    let deck_time = key
                        .parse::<usize>()
                        .map_err(de::Error::custom)?;
                    let deck_config: DeckConfig =
                        serde_json::from_str(&value)
                            .map_err(de::Error::custom)?;
                    deck_configs.push((deck_time, deck_config));
                }
                Ok(deck_configs)
            }
        }
        deserializer.deserialize_map(CollectionVisitor)
    }

    pub fn new() -> Self {
        Self {
            id: 0,
            created: 0,
            modified: 0,
            scheme_mod_time: 0,
            version: 0,
            dirty: 0,
            update_seq_number: 0,
            last_sync_time: 0,
            config: CollectionConfig::new(),
            models: vec![],
            decks: vec![],
            deck_configs: vec![],
            tags: String::new(),
        }
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}
