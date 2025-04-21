#[derive(Debug, Clone, Default)]
pub struct SimpleInformation {
    pub word: String,
    pub definitions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_information() {
        let simple_info = SimpleInformation {
            word: "hello".to_string(),
            definitions: vec!["a greeting".to_string()],
        };

        assert_eq!(simple_info.word, "hello");
        assert_eq!(simple_info.definitions, vec!["a greeting"])
    }
}
