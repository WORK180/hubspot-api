use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteProperties {
    /// The note's text content, limited to 65,536 characters.
    #[serde(rename = "hs_note_body")]
    pub body: String,
    /// This field marks the note's time of creation and
    /// determines where the note sits on the record timeline.
    #[serde(rename = "hs_timestamp", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}

impl NoteProperties {
    pub fn new(body: String) -> Self {
        Self {
            body,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}
