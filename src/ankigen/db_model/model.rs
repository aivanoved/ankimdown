use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum ModelType {
    FrontBack = 0,
    Cloze = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ModelField {
    pub font: String, // display font
    #[serde(default)]
    pub media: Vec<String>, // appears to be unused
    pub name: String, // field name
    #[serde(rename = "ord")]
    pub ordinal: usize, // ordinal of the field
    #[serde(rename = "rtl")]
    pub right_to_left: bool, // right to left
    #[serde(rename = "size")]
    pub font_size: usize, // font size
    pub sticky: bool, // stocky fields retain the last added value
}

impl ModelField {
    pub fn new(name: String, ordinal: usize) -> Self {
        Self {
            font: "Arial".to_string(),
            media: vec![],
            name,
            ordinal,
            right_to_left: false,
            font_size: 20,
            sticky: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ModelTemplate {
    #[serde(rename = "afmt")]
    pub answer_template: String, // answer tempalate
    #[serde(rename = "bafmt")]
    pub browser_answer_template: String, // browser answer format
    #[serde(rename = "bqfmt")]
    pub browser_question_format: String, // browser question format
    #[serde(rename = "did")]
    pub default_deck_id: Option<u64>, // default deck id, default None
    pub name: String, // name of the template
    #[serde(rename = "qfmt")]
    pub question_format: String, // question format
}

impl ModelTemplate {
    pub fn new(name: String, question_format: String, answer_template: String) -> Self {
        Self {
            answer_template,
            browser_answer_template: String::new(),
            browser_question_format: String::new(),
            default_deck_id: None,
            name,
            question_format,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Model {
    #[serde(default = "Model::default_css")]
    pub css: String, // css shared between the templates
    #[serde(rename = "did", default = "Model::default_deck_id")]
    pub default_deck_id: u64, // default deck id to add cards to
    #[serde(rename = "flds")]
    pub fields: Vec<ModelField>, // fields in the model
    #[serde(rename = "id")]
    pub model_id: u64, // model id, used to identify the model, see notes.mid
    #[serde(rename = "latexPost", default = "Model::default_latex_post")]
    pub latex_post: String, // latex postamble
    #[serde(rename = "latexPre", default = "Model::default_latex_pre")]
    pub latex_pre: String, // latex preamble
    #[serde(rename = "mod", default = "Model::default_modification")]
    pub modification: i64, // modification time
    pub name: String, // name of the model
    #[serde(default = "Model::default_req")]
    pub req: Vec<(usize, String, Vec<usize>)>, // req is unused in modern clients
    #[serde(rename = "sortf")]
    pub sort_field_index: usize, // which field is used for sorting
    #[serde(default = "Model::default_tags")]
    pub tags: Vec<String>, // tags for the model, anki manages this, use [] when creating
    #[serde(rename = "tmpls")]
    pub templates: Vec<ModelTemplate>, // templates for the model
    #[serde(rename = "type")]
    pub model_type: ModelType, // type of model, see ModelType
    #[serde(rename = "usn", default = "Model::default_usn")]
    pub update_seq_number: i64, // usn is the last sync number, anki manages this, use -1 when creating
    #[serde(rename = "vers", default = "Model::default_version")]
    pub version: Vec<u8>, // version of the model, anki manages this, use [] when creating
}

impl Model {
    pub fn default_css() -> String {
        r#".card {
    font-family: arial;
    font-size: 20px;
    text-align: center;
    color: black;
    background-color: white;
}"#
        .to_string()
    }

    pub fn default_deck_id() -> u64 {
        0
    }

    pub fn default_latex_pre() -> String {
        r#"
\documentclass[12pt]{article}
\special{papersize=3in,5in}
\usepackage[utf8]{inputenc}
\usepackage{amssymb,amsmath}
\pagestyle{empty}
\setlength{\parindent}{0in}
\begin{document}
"#
        .to_string()
    }

    pub fn default_latex_post() -> String {
        r#"\end{document}"#.to_string()
    }

    pub fn default_modification() -> i64 {
        0
    }

    pub fn default_usn() -> i64 {
        -1
    }

    pub fn default_req() -> Vec<(usize, String, Vec<usize>)> {
        vec![]
    }

    pub fn default_tags() -> Vec<String> {
        vec![]
    }

    pub fn default_version() -> Vec<u8> {
        vec![]
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        css: Option<String>,
        fields: Vec<ModelField>,
        model_id: u64,
        latex_post: Option<String>,
        latex_pre: Option<String>,
        name: String,
        sort_field_index: usize,
        templates: Vec<ModelTemplate>,
        model_type: ModelType,
    ) -> Self {
        Self {
            css: css.unwrap_or_else(Self::default_css),
            default_deck_id: Self::default_deck_id(),
            fields,
            model_id,
            latex_post: latex_post.unwrap_or_else(Self::default_latex_post),
            latex_pre: latex_pre.unwrap_or_else(Self::default_latex_pre),
            modification: Self::default_modification(),
            name,
            req: Self::default_req(),
            sort_field_index,
            tags: Self::default_tags(),
            templates,
            model_type,
            update_seq_number: Self::default_usn(),
            version: Self::default_version(),
        }
    }
}
