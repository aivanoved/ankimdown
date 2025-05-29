use ankimdown::db_model::card::*;

#[test]
fn test_card_flag() {
    assert_eq!(CardFlag::try_from(0).unwrap(), CardFlag::Null);
    assert_eq!(CardFlag::try_from(1).unwrap(), CardFlag::Redo);
    assert_eq!(CardFlag::try_from(2).unwrap(), CardFlag::Orange);
    assert_eq!(CardFlag::try_from(3).unwrap(), CardFlag::Green);
    assert_eq!(CardFlag::try_from(4).unwrap(), CardFlag::Blue);
    assert!(CardFlag::try_from(5).is_err());
}

#[test]
fn test_card_new() {
    let card = Card::new(
        1,
        2,
        3,
        4,
        5,
        6,
        CardType::New,
        CardQueue::New,
        7,
        8,
        9,
        10,
        11,
        12,
        0,
        13,
        CardFlag::Null,
    );
    assert_eq!(card.id, 1);
    assert_eq!(card.note_id, 2);
    assert_eq!(card.deck_id, 3);
    assert_eq!(card.ordinal, 4);
    assert_eq!(card.modified, 5);
    assert_eq!(card.update_seq_number, 6);
    assert_eq!(card.card_type, CardType::New);
    assert_eq!(card.queue, CardQueue::New);
    assert_eq!(card.due, 7);
    assert_eq!(card.interval, 8);
    assert_eq!(card.factor, 9);
    assert_eq!(card.reviews, 10);
    assert_eq!(card.lapses, 11);
    assert_eq!(card.left, 12);
    assert_eq!(card.original_due, 0);
    assert_eq!(card.original_deck_id, 13);
    assert_eq!(card.flags, CardFlag::Null);
}

#[test]
fn test_card_serialization() {
    let card = Card::new(
        1,
        2,
        3,
        4,
        5,
        6,
        CardType::New,
        CardQueue::New,
        7,
        8,
        9,
        10,
        11,
        12,
        0,
        13,
        CardFlag::Null,
    );
    let serialized = serde_json::to_string(&card).unwrap();
    let deserialized: Card =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(card, deserialized);
}
