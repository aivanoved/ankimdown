use ankimdown::db_model::model::*;

#[test]
fn test_model_field() {
    let field = ModelField::new("test".to_string(), 1);
    assert_eq!(field.font, "Arial");
    assert_eq!(field.media.len(), 0);
    assert_eq!(field.name, "test");
    assert_eq!(field.ordinal, 1);
    assert!(!field.right_to_left);
    assert_eq!(field.font_size, 20);
    assert!(!field.sticky);
}

#[test]
fn test_model_template() {
    let template = ModelTemplate::new(
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
    );
    assert_eq!(template.answer_template, "test");
    assert_eq!(template.browser_answer_template, "");
    assert_eq!(template.browser_question_format, "");
    assert_eq!(template.default_deck_id, None);
    assert_eq!(template.name, "test");
    assert_eq!(template.question_format, "test");
}

#[test]
fn test_model() {
    let model = Model::new(
        Some("test".to_string()),
        vec![],
        1,
        Some("test".to_string()),
        Some("test".to_string()),
        "test".to_string(),
        0,
        vec![],
        ModelType::FrontBack,
    );

    assert_eq!(model.css, "test");
    assert_eq!(model.fields.len(), 0);
    assert_eq!(model.model_id, 1);
    assert_eq!(model.latex_post, "test");
    assert_eq!(model.latex_pre, "test");
    assert_eq!(model.modification, 0);
    assert_eq!(model.name, "test");
    assert_eq!(model.sort_field_index, 0);
    assert_eq!(model.templates.len(), 0);
    assert_eq!(model.model_type, ModelType::FrontBack);
}

#[test]
fn test_model_default() {
    let model = Model::new(
        None,
        vec![],
        1,
        None,
        None,
        "test".to_string(),
        0,
        vec![],
        ModelType::FrontBack,
    );
    assert_eq!(model.css, Model::default_css());
    assert_eq!(model.fields.len(), 0);
    assert_eq!(model.model_id, 1);
    assert_eq!(model.latex_post, Model::default_latex_post());
    assert_eq!(model.latex_pre, Model::default_latex_pre());
    assert_eq!(model.modification, 0);
    assert_eq!(model.name, "test");
    assert_eq!(model.sort_field_index, 0);
    assert_eq!(model.templates.len(), 0);
    assert_eq!(model.model_type, ModelType::FrontBack);
}

#[test]
fn test_serialize() {
    let model = Model::new(
        Some("test".to_string()),
        vec![],
        1,
        Some("test".to_string()),
        Some("test".to_string()),
        "test".to_string(),
        0,
        vec![],
        ModelType::FrontBack,
    );
    let serialized = serde_json::to_string(&model).unwrap();
    let deserialized: Model =
        serde_json::from_str(&serialized).unwrap();
    assert_eq!(model, deserialized);
}
