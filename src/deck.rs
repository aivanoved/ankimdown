#[allow(dead_code)]
#[derive(Debug, Clone)]
struct DeckAutoMeta {
    id: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct DeckMeta {
    autogen: DeckAutoMeta,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Deck {
    name: String,
    description: String,
    metadata: DeckMeta,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_auto_meta() {
        let auto_meta = DeckAutoMeta { id: 1 };
        assert_eq!(auto_meta.id, 1);
    }

    #[test]
    fn test_deck_meta() {
        let auto_meta = DeckAutoMeta { id: 1 };
        let deck_meta = DeckMeta { autogen: auto_meta };
        assert_eq!(deck_meta.autogen.id, 1);
    }

    #[test]
    fn test_deck() {
        let deck = Deck {
            name: "Test Deck".to_string(),
            description: "This is a test deck".to_string(),
            metadata: DeckMeta {
                autogen: DeckAutoMeta { id: 1 },
            },
        };
        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.description, "This is a test deck");
        assert_eq!(deck.metadata.autogen.id, 1);
    }
}
