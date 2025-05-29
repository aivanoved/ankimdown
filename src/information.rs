use crate::ankigen::db_model::card::Card;
use crate::markdown::ast::Node;

type Example = Vec<String>;

#[derive(Debug, Clone, Default)]
pub struct InformationContent {
    pub word: String,
    pub definitions: Vec<String>,
    pub examples: Vec<Option<Example>>,
}

pub trait IntoCards {
    fn into_cards(
        &self,
        information_content: &InformationContent,
    ) -> Vec<Card>;
}

pub struct Information {
    pub information_type: Box<dyn IntoCards>,
    pub content: InformationContent,
}

impl Information {
    pub fn from_ast(node: &Node) -> Self {
        todo!()
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.information_type.into_cards(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_information() {
        let simple_info = InformationContent {
            word: "hello".to_string(),
            definitions: vec!["a greeting".to_string()],
            examples: vec![None],
        };

        assert_eq!(simple_info.word, "hello");
        assert_eq!(simple_info.definitions, vec!["a greeting"])
    }
}
