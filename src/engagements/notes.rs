use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Properties for a HubSpot Note engagement.
///
/// Use [`NoteProperties::new()`] to construct with the current timestamp set automatically.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    /// Creates a new [`NoteProperties`] with the given body text.
    /// Sets `timestamp` to the current UTC time.
    pub fn new(body: String) -> Self {
        Self {
            body,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}
