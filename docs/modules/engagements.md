# Module: engagements

Sources: `src/engagements/mod.rs`, `src/engagements/notes.rs`

Provides the `EngagementType` enum, `EngagementsManager` struct, and per-engagement-type properties structs. Currently supports `Notes`; the design is intentionally extensible to tasks, meetings, calls, and emails.

---

## `EngagementType`

```rust
pub enum EngagementType {
    Notes,
}
```

Implements:
- `ToPath` — maps each variant to its HubSpot engagement URL path segment
- `Display` (via `strum_macros::Display`)
- `Clone`, `Debug`

### `ToPath` Mapping

| Variant | `to_path()` output |
|---|---|
| `Notes` | `"notes"` |

---

## `EngagementsManager`

```rust
pub struct EngagementsManager {
    pub notes: ApiCollection<EngagementType>,
}
```

Owned by `Hubspot`. Each field is an `ApiCollection<EngagementType>` pre-configured for that engagement type. All fields expose the full CRUD + batch + associations surface:

```rust
hubspot.engagements.notes.create(note_record).await?;
hubspot.engagements.notes.read("789", false).await?;
hubspot.engagements.notes.associations.create("789", "456", AssociationLinks::NoteToContact).await?;
```

---

## `NoteProperties`

Source: `src/engagements/notes.rs`

```rust
pub struct NoteProperties {
    #[serde(rename = "hs_note_body")]
    pub body: String,

    #[serde(rename = "hs_timestamp", with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
}
```

### Constructor

```rust
impl NoteProperties {
    pub fn new(body: String) -> Self
}
```

Sets `timestamp` to `OffsetDateTime::now_utc()` automatically. Use this for creating new notes.

### Example

```rust
use hubspot::notes::NoteProperties;
use hubspot::types::{HubspotRecord, OptionNotDesired, AssociationLinks};

let mut note = HubspotRecord::<NoteProperties, OptionNotDesired, OptionNotDesired>
    ::with_properties_and_associations(NoteProperties::new("Follow-up completed.".into()));

note.attach_built_in_associations(AssociationLinks::NoteToContact, vec!["456".to_string()]);
note.attach_built_in_associations(AssociationLinks::NoteToDeal, vec!["123".to_string()]);

let created = hubspot.engagements.notes.create(note).await?;
```

---

## Built-in Association Types for Engagements

Defined in `src/api_configs/types.rs` as `AssociationLinks`:

| Variant | Numeric ID | Meaning |
|---|---|---|
| `NoteToContact` | 202 | Associate a note with a contact |
| `NoteToCompany` | 190 | Associate a note with a company |
| `NoteToDeal` | 214 | Associate a note with a deal |

---

## Adding a New Engagement Type

See [docs/architecture.md](../architecture.md#new-engagement-type) for the full guide. In brief:

1. **Add variant** to `EngagementType` in `src/engagements/mod.rs`
2. **Add `to_path()` arm** in the `ToPath` impl
3. **Create `src/engagements/<type>.rs`** with a `<Type>Properties` struct following the same pattern as `NoteProperties`:
   - `Serialize + Deserialize + Debug`
   - Use `#[serde(rename = "hs_...")]` to match HubSpot field names
   - Use `#[serde(with = "time::serde::rfc3339")]` for any timestamp fields
4. **Add `ApiCollection<EngagementType>` field** to `EngagementsManager`
5. **Wire in `EngagementsManager::new()`**
