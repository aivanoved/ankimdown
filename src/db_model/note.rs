use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteTag {
    pub name: String,
}

impl NoteTag {
    pub fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Tag name cannot be empty".to_string());
        } else if name.contains("__") {
            return Err("Tag name cannot contain '__'".to_string());
        } else if name.contains(' ') {
            return Err("Tag name cannot contain spaces".to_string());
        } else if name.len() > 255 {
            return Err("Tag name cannot be longer than 255 characters".to_string());
        }

        Ok(NoteTag {
            name: name.to_string(),
        })
    }
}

impl Default for NoteTag {
    fn default() -> Self {
        NoteTag {
            name: "default_tag".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: i64, // time in milliseconds when the note was created
    #[serde(rename = "guid")]
    pub global_id: String, // unique identifier for the note
    #[serde(rename = "mid")]
    pub model_id: usize, // model id of the note
    #[serde(rename = "mod")]
    pub modified: i64, // time in milliseconds when the note was last modified
    #[serde(rename = "usn")]
    pub update_seq_number: i64,
    #[serde(default)]
    pub tags: Vec<NoteTag>, // tags associated with the note
    #[serde(rename = "flds")]
    pub fields: Vec<String>,
    #[serde(rename = "sfld")]
    pub sort_filed: usize, // index of the field used for sorting
    #[serde(rename = "csum")]
    pub checksum: i64, // checksum of the note, the first 8 digits of sha1 hash of the first field
    pub flags: i64,   // flags associated with the note, unused
    pub data: String, // additional data associated with the note, unused
}

impl Note {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        global_id: String,
        model_id: usize,
        modified: i64,
        update_seq_number: i64,
        tags: Vec<NoteTag>,
        fields: Vec<String>,
        sort_filed: usize,
        checksum: i64,
    ) -> Self {
        Note {
            id,
            global_id,
            model_id,
            modified,
            update_seq_number,
            tags,
            fields,
            sort_filed,
            checksum,
            flags: 0,
            data: "".to_string(),
        }
    }
}
