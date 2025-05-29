use ankimdown::db_model::collection::*;
use ankimdown::db_model::deck::{Deck, DeckConfig};
use ankimdown::db_model::model::{Model, ModelType};

fn create_test_model() -> Model {
    Model::new(
        None,
        vec![],
        1,
        None,
        None,
        "Test Model".to_string(),
        0,
        vec![],
        ModelType::FrontBack,
    )
}
fn create_test_deck() -> Deck {
    Deck::new("Test Deck".to_string())
}
fn create_test_deck_config() -> DeckConfig {
    DeckConfig::new("Test Deck Config".to_string())
}

#[test]
fn test_new_spread_serialization() {
    let new_spread = NewSpread::NewCardsDistribute;
    let serialized = serde_json::to_string(&new_spread).unwrap();
    let deserialized: NewSpread =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(new_spread, deserialized);

    let new_spread = NewSpread::NewCardsLast;
    let serialized = serde_json::to_string(&new_spread).unwrap();
    let deserialized: NewSpread =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(new_spread, deserialized);

    let new_spread = NewSpread::NewCardsFirst;
    let serialized = serde_json::to_string(&new_spread).unwrap();
    let deserialized: NewSpread =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(new_spread, deserialized);
}

#[test]
fn test_collection_config() {
    let config = CollectionConfig::new();
    assert_eq!(config.current_deck, 0);
    assert_eq!(config.active_decks, Vec::<i64>::new());
    assert_eq!(config.new_spread, NewSpread::NewCardsDistribute);
    assert_eq!(config.collapse_time, 0);
    assert_eq!(config.time_limit, 0);
    assert_eq!(config.estimated_times, false);
    assert_eq!(config.due_counts, false);
    assert_eq!(config.current_model, "");
    assert_eq!(config.next_position, 0);
    assert_eq!(config.sort_type, "");
    assert_eq!(config.sort_backwards, false);
    assert_eq!(config.add_to_current, false);
    assert_eq!(config.day_learn_first, false);
    assert_eq!(config.new_bury, true);
}

#[test]
fn test_collection_config_serialization() {
    let config = CollectionConfig::new();
    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CollectionConfig =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(config.current_deck, deserialized.current_deck);
    assert_eq!(config.active_decks, deserialized.active_decks);
}

#[test]
fn test_collection() {
    let collection = Collection::new();
    assert_eq!(collection.id, 0);
    assert_eq!(collection.created, 0);
    assert_eq!(collection.modified, 0);
    assert_eq!(collection.scheme_mod_time, 0);
    assert_eq!(collection.version, 0);
    assert_eq!(collection.dirty, 0);
    assert_eq!(collection.update_seq_number, 0);
    assert_eq!(collection.last_sync_time, 0);
    assert_eq!(collection.config, CollectionConfig::new());
    assert_eq!(collection.models, Vec::<(usize, Model)>::new());
    assert_eq!(collection.decks, Vec::<(usize, Deck)>::new());
    assert_eq!(
        collection.deck_configs,
        Vec::<(usize, DeckConfig)>::new()
    );
    assert_eq!(collection.tags, "");
}

#[test]
fn test_collection_serialization() {
    let collection = Collection {
        id: 1,
        created: 1234567890,
        modified: 1234567890,
        scheme_mod_time: 1234567890,
        version: 1,
        dirty: 0,
        update_seq_number: 1,
        last_sync_time: 1234567890,
        config: CollectionConfig::new(),
        models: vec![(1234567890, create_test_model())],
        decks: vec![(1234567890, create_test_deck())],
        deck_configs: vec![(1234567890, create_test_deck_config())],
        tags: String::new(),
    };
    let serialized = serde_json::to_string(&collection).unwrap();
    let deserialized: Collection =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(collection.id, deserialized.id);
    assert_eq!(collection.created, deserialized.created);
    assert_eq!(collection.modified, deserialized.modified);
    assert_eq!(
        collection.scheme_mod_time,
        deserialized.scheme_mod_time
    );
    assert_eq!(collection.version, deserialized.version);
    assert_eq!(collection.dirty, deserialized.dirty);
    assert_eq!(
        collection.update_seq_number,
        deserialized.update_seq_number
    );
    assert_eq!(
        collection.last_sync_time,
        deserialized.last_sync_time
    );
    assert_eq!(collection.config, deserialized.config);
    assert_eq!(collection.models.len(), deserialized.models.len());
    assert_eq!(collection.models[0].0, deserialized.models[0].0);
    assert_eq!(collection.models[0].1, deserialized.models[0].1);
    assert_eq!(collection.decks.len(), deserialized.decks.len());
    assert_eq!(collection.decks[0].0, deserialized.decks[0].0);
    assert_eq!(collection.decks[0].1, deserialized.decks[0].1);
    assert_eq!(
        collection.deck_configs.len(),
        deserialized.deck_configs.len()
    );
    assert_eq!(
        collection.deck_configs[0].0,
        deserialized.deck_configs[0].0
    );
    assert_eq!(
        collection.deck_configs[0].1,
        deserialized.deck_configs[0].1
    );
    assert_eq!(collection.tags, deserialized.tags);
}
