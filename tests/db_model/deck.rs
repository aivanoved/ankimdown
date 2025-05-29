use ankimdown::ankigen::db_model::deck::*;
use serde_json::json;

#[test]
fn test_deck_init() {
    let deck = Deck::new("Test Deck".to_string());
    assert_eq!(deck.name, "Test Deck");
    assert_eq!(deck.extended_review_limit, None);
    assert_eq!(deck.update_seq_number, 0);
    assert!(!deck.collapsed);
    assert!(!deck.browser_collapsed);
    assert_eq!(deck.new_today, [0, 0]);
    assert_eq!(deck.review_today, [0, 0]);
    assert_eq!(deck.learn_today, [0, 0]);
    assert_eq!(deck.time_today, [0, 0]);
    assert_eq!(deck.filtered, 0);
    assert_eq!(deck.extended_new_limit, None);
    assert_eq!(deck.config_id, None);
    assert_eq!(deck.id, 0);
    assert_eq!(deck.modified, 0);
    assert_eq!(deck.description, "");
}

#[test]
fn test_deck_serialize() {
    let deck = Deck::new("Test Deck".to_string());
    let serialized = serde_json::to_string(&deck).unwrap();
    let expected = json!({
        "name": "Test Deck",
        "extendRev": null,
        "usn": 0,
        "collapsed": false,
        "browserCollapsed": false,
        "newToday": [0, 0],
        "revToday": [0, 0],
        "lrnToday": [0, 0],
        "timeToday": [0, 0],
        "dyn": 0,
        "extendNew": null,
        "conf": null,
        "id": 0,
        "mod": 0,
        "desc": ""
    });
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
        expected
    );
}

#[test]
fn test_deck_deserialize() {
    let data = json!({
        "name": "Test Deck",
        "extendRev": null,
        "usn": 0,
        "collapsed": false,
        "browserCollapsed": false,
        "newToday": [0, 0],
        "revToday": [0, 0],
        "lrnToday": [0, 0],
        "timeToday": [0, 0],
        "dyn": 0,
        "extendNew": null,
        "conf": null,
        "id": 0,
        "mod": 0,
        "desc": ""
    });
    let deck: Deck = serde_json::from_value(data).unwrap();
    assert_eq!(deck.name, "Test Deck");
    assert_eq!(deck.extended_review_limit, None);
    assert_eq!(deck.update_seq_number, 0);
    assert!(!deck.collapsed);
    assert!(!deck.browser_collapsed);
    assert_eq!(deck.new_today, [0, 0]);
    assert_eq!(deck.review_today, [0, 0]);
    assert_eq!(deck.learn_today, [0, 0]);
    assert_eq!(deck.time_today, [0, 0]);
    assert_eq!(deck.filtered, 0);
    assert_eq!(deck.extended_new_limit, None);
    assert_eq!(deck.config_id, None);
    assert_eq!(deck.id, 0);
    assert_eq!(deck.modified, 0);
    assert_eq!(deck.description, "");
}

#[test]
fn test_lapse_config_init() {
    let lapse_config = LapseConfig::new();
    assert_eq!(lapse_config.delays, vec![0]);
    assert_eq!(lapse_config.leech_action, 0);
    assert_eq!(lapse_config.leech_fails, 0);
    assert_eq!(lapse_config.min_interval, 0);
    assert_eq!(lapse_config.interval_increase, 0);
}

#[test]
fn test_new_config_init() {
    let new_config = NewConfig::new();
    assert!(!new_config.bury);
    assert_eq!(new_config.delays, vec![0]);
    assert_eq!(new_config.initial_factor, 0);
    assert_eq!(new_config.intervals, vec![0]);
    assert_eq!(new_config.order, NewCardOrder::Random);
    assert_eq!(new_config.per_day, 0);
    assert!(!new_config.separate);
}

#[test]
fn test_review_config_init() {
    let review_config = ReviewConfig::new();
    assert!(!review_config.bury);
    assert_eq!(review_config.ease_factor, 0);
    assert_eq!(review_config.fuzz, 0);
    assert_eq!(review_config.interval_factor, 0);
    assert_eq!(review_config.max_interval, 0);
    assert_eq!(review_config.min_space, 0);
    assert_eq!(review_config.cards_daily, 0);
}

#[test]
fn test_deck_config_init() {
    let deck_config = DeckConfig::new("Test Deck".to_string());
    assert!(!deck_config.autoplay);
    assert_eq!(deck_config.filtered, None);
    assert_eq!(deck_config.id, None);
    assert_eq!(deck_config.lapse_config.delays, vec![0]);
    assert_eq!(deck_config.max_taken, 0);
    assert_eq!(deck_config.modified, 0);
    assert_eq!(deck_config.name, "Test Deck");
    assert!(!deck_config.new_config.bury);
    assert!(!deck_config.replay_question);
    assert!(!deck_config.review_config.bury);
    assert!(!deck_config.timer);
    assert_eq!(deck_config.update_seq_number, 0);
}

#[test]
fn test_deck_config_serialize() {
    let deck_config = DeckConfig::new("Test Deck".to_string());
    let serialized = serde_json::to_string(&deck_config).unwrap();
    let expected = json!({
        "autoplay": false,
        "dyn": null,
        "id": null,
        "lapse": {
            "delays": [0],
            "leechAction": 0,
            "leechFails": 0,
            "minInt": 0,
            "mult": 0
        },
        "maxTaken": 0,
        "mod": 0,
        "name": "Test Deck",
        "new": {
            "bury": false,
            "delays": [0],
            "initialFactor": 0,
            "ints": [0],
            "order": 0,
            "perDay": 0,
            "separate": false
        },
        "replayq": false,
        "rev": {
            "bury": false,
            "ease4": 0,
            "fuzz": 0,
            "ivlFct": 0,
            "maxIvl": 0,
            "minSpace": 0,
            "perDay": 0
        },
        "timer": false,
        "usn": 0
    });
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
        expected
    );
}

#[test]
fn test_deck_config_deserialize() {
    let data = json!({
        "autoplay": false,
        "dyn": null,
        "id": null,
        "lapse": {
            "delays": [0],
            "leechAction": 0,
            "leechFails": 0,
            "minInt": 0,
            "mult": 0
        },
        "maxTaken": 0,
        "mod": 0,
        "name": "Test Deck",
        "new": {
            "bury": false,
            "delays": [0],
            "initialFactor": 0,
            "ints": [0],
            "order": 0,
            "perDay": 0,
            "separate": false
        },
        "replayq": false,
        "rev": {
            "bury": false,
            "ease4": 0,
            "fuzz": 0,
            "ivlFct": 0,
            "maxIvl": 0,
            "minSpace": 0,
            "perDay": 0
        },
        "timer": false,
        "usn": 0
    });
    let deck_config: DeckConfig = serde_json::from_value(data).unwrap();
    assert_eq!(deck_config.name, "Test Deck");
    assert!(!deck_config.autoplay);
    assert_eq!(deck_config.filtered, None);
    assert_eq!(deck_config.id, None);
    assert_eq!(deck_config.lapse_config.delays, vec![0]);
    assert_eq!(deck_config.max_taken, 0);
    assert_eq!(deck_config.modified, 0);
    assert!(!deck_config.new_config.bury);
    assert!(!deck_config.replay_question);
    assert!(!deck_config.review_config.bury);
    assert!(!deck_config.timer);
    assert_eq!(deck_config.update_seq_number, 0);
}
