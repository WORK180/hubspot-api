# API Reference — hubspot-api

Consumer-facing guide for integrating the `hubspot` crate into your Rust project.

---

## Installation

```toml
[dependencies]
hubspot = "0.2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1", features = ["derive"] }
```

---

## Authentication

This crate uses HubSpot **Private App tokens** exclusively. No OAuth.

1. In HubSpot: Settings → Integrations → Private Apps → Create a private app
2. Copy the token (starts with `pat-...`)
3. Grant the scopes your app needs (e.g. `crm.objects.contacts.read`)

```sh
# .env (for local development)
HUBSPOT_TOKEN=pat-na1-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
HUBSPOT_DOMAIN=api.hubapi.com
HUBSPOT_PORTAL_ID=12345678
```

---

## Client Initialization

```rust
use hubspot::Hubspot;

let hubspot = Hubspot::builder()
    .domain("api.hubapi.com")
    .token("pat-na1-...")   // note: NOT .key() — the README has a typo
    .portal_id("12345678")
    .build()
    .expect("Failed to build Hubspot client");
```

> **Note:** `.token()` is the correct builder method. An older version of the README shows `.key()` which does not exist.

You may also inject a custom `reqwest::Client`:

```rust
let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()?;

let hubspot = Hubspot::builder()
    .domain("api.hubapi.com")
    .token("pat-na1-...")
    .portal_id("12345678")
    .client(&client)          // takes &reqwest::Client
    .build()?;
```

---

## Defining Properties

Define a struct for the HubSpot properties you want returned. The crate automatically maps field names to `?properties=` query parameters via `serde_introspect`.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DealProperties {
    #[serde(rename = "dealname")]
    name: String,
    #[serde(rename = "amount")]
    amount: Option<String>,
    #[serde(rename = "closedate")]
    close_date: Option<String>,
}
```

---

## CRM Objects

Available object types: `Contacts`, `Companies`, `Deals`, `LineItems`.

### Read (Get by ID)

```rust
use hubspot::types::{HubspotRecord, OptionNotDesired};

let deal: HubspotRecord<DealProperties, OptionNotDesired, OptionNotDesired> =
    hubspot.objects.deals.read("123", false).await?;

println!("{}", deal.properties.name);
```

### List

`list` takes `archived: Option<bool>` — pass `Some(false)` or `None` (defaults to false).

```rust
use hubspot::types::OptionNotDesired;

let results = hubspot.objects.deals
    .list::<DealProperties, OptionNotDesired, OptionNotDesired>(
        Some(10),       // limit
        None,           // after cursor (pagination)
        Some(false),    // archived
    )
    .await?;

for deal in results.results {
    println!("{}: {:?}", deal.id, deal.properties.name);
}

// Pagination cursor for next page:
if let Some(paging) = results.paging {
    let next_cursor = paging.next.after;
}
```

### Create

Use `with_properties_and_associations` — this initialises the associations vec required by the v4 create endpoint.

```rust
use hubspot::types::{HubspotRecord, OptionNotDesired};

let payload =
    HubspotRecord::with_properties_and_associations(DealProperties {
        name: "New Deal".to_string(),
        amount: Some("5000".to_string()),
        close_date: None,
    });

let created = hubspot.objects.deals
    .create::<DealProperties, OptionNotDesired, OptionNotDesired>(payload)
    .await?;
println!("Created deal ID: {}", created.id);
```

### Update

`update` takes the record ID and the properties struct directly.

```rust
let updated = hubspot.objects.deals
    .update::<DealProperties, OptionNotDesired>(
        "123".to_string(),
        DealProperties {
            name: "Updated Deal Name".to_string(),
            amount: None,
            close_date: None,
        },
    )
    .await?;
```

### Archive (Soft Delete)

```rust
hubspot.objects.deals.archive("123".to_string()).await?;
```

---

## Batch Operations

### Batch Read

`batch.read` takes explicit property/association struct instances alongside the IDs.

```rust
use hubspot::types::OptionNotDesired;

let result = hubspot.objects.deals
    .batch
    .read::<DealProperties, OptionNotDesired, OptionNotDesired>(
        vec!["123", "456", "789"],
        DealProperties { name: String::new(), amount: None, close_date: None },
        OptionNotDesired {},
        OptionNotDesired {},
        Some(false),
    )
    .await?;

for deal in result.results {
    println!("{}: {}", deal.id, deal.properties.name);
}
```

### Batch Create

`batch.create` takes a `Vec` of your properties struct (not `HubspotRecord`).

```rust
let result = hubspot.objects.deals
    .batch
    .create(vec![
        DealProperties { name: "Deal A".to_string(), amount: None, close_date: None },
        DealProperties { name: "Deal B".to_string(), amount: None, close_date: None },
    ])
    .await?;
```

### Batch Update

`batch.update` applies the same properties to all record IDs.

```rust
use hubspot::types::OptionNotDesired;

let result = hubspot.objects.deals
    .batch
    .update::<DealProperties, OptionNotDesired>(
        vec!["123".to_string(), "456".to_string()],
        DealProperties { name: "New Name".to_string(), amount: None, close_date: None },
    )
    .await?;
```

### Batch Archive

```rust
hubspot.objects.deals.batch.archive(vec!["123", "456"]).await?;
```

---

## Associations

### List Associations

`list` takes `to_object_type` as a `&str` path segment (e.g. `"contacts"`).

```rust
let associations = hubspot.objects.deals
    .associations
    .list("123", "contacts", None, None)
    .await?;

for assoc in associations.results {
    println!("Contact ID: {}, type_id: {}", assoc.to_object_id, assoc.association_types[0].type_id);
}
```

### Create Association

`create` takes `to_object_type` (any type implementing `ToPath`), the target ID, and a vec of `AssociationCreationDetails`.

```rust
use hubspot::associations::AssociationCreationDetails;
use hubspot::ObjectType;

hubspot.objects.deals
    .associations
    .create(
        "123",
        ObjectType::Contacts,
        "456",
        vec![AssociationCreationDetails {
            category: "HUBSPOT_DEFINED".to_string(),
            type_id: 3,  // Deal → Contact built-in type ID
        }],
    )
    .await?;
```

### Delete Association

```rust
use hubspot::ObjectType;

hubspot.objects.deals
    .associations
    .delete("123", ObjectType::Contacts, "456")
    .await?;
```

---

## Engagements — Notes

```rust
use hubspot::notes::NoteProperties;
use hubspot::types::{HubspotRecord, AssociationLinks};

let note = HubspotRecord::with_properties_and_associations(
    NoteProperties::new("Call went well.".to_string()),
);
let note = note.attach_built_in_associations(
    AssociationLinks::NoteToContact,
    vec!["456".to_string()],
);
let note = note.attach_built_in_associations(
    AssociationLinks::NoteToDeal,
    vec!["123".to_string()],
);

let created = hubspot.engagements.notes.create(note).await?;
println!("Note ID: {}", created.id);
```

`NoteProperties::new(body)` automatically sets `hs_timestamp` to the current UTC time.

---

## Owners

`read` takes `archived: Option<bool>`.

```rust
let owner = hubspot.owners.read("12345678", Some(false)).await?;
println!("{} {} <{}>", owner.first_name, owner.last_name, owner.email);

if let Some(teams) = owner.teams {
    for team in teams {
        println!("Team: {} (primary: {})", team.name, team.primary);
    }
}
```

---

## Reading with PropertiesWithHistory

`properties_with_history` is always present (populated via `#[serde(default)]`), not an `Option`.

```rust
#[derive(Debug, Default, Deserialize)]
struct DealHistory {
    #[serde(rename = "amount")]
    amount: Option<Vec<serde_json::Value>>,
}

let deal = hubspot.objects.deals
    .read::<DealProperties, DealHistory, OptionNotDesired>("123", false)
    .await?;

println!("{:?}", deal.properties_with_history.amount);
```

---

## Reading with Associations Inline

`associations` is always present (populated via `#[serde(default)]`). Implement `Default` on your associations struct.

```rust
#[derive(Debug, Default, Deserialize)]
struct DealAssociations {
    #[serde(default)]
    contacts: hubspot::types::AssociationResults,
}

let deal = hubspot.objects.deals
    .read::<DealProperties, OptionNotDesired, DealAssociations>("123", false)
    .await?;

for contact in deal.associations.contacts.results {
    println!("Associated contact ID: {}", contact.id);
}
```

---

## Error Handling

`HubspotError` is re-exported from the crate root.

```rust
use hubspot::HubspotError;

match hubspot.objects.deals.read::<DealProperties, _, _>("bad-id", false).await {
    Ok(deal) => println!("{}", deal.properties.name),
    Err(HubspotError::Http(e)) => eprintln!("HTTP error: {e}"),
    Err(HubspotError::Json(e)) => eprintln!("JSON parse error: {e}"),
    Err(HubspotError::Hubspot(msg)) => eprintln!("HubSpot API error: {msg}"),
}
```

| Variant | When |
|---|---|
| `HubspotError::Http(reqwest::Error)` | Network failure, timeout, non-success HTTP status |
| `HubspotError::Json(serde_json::Error)` | Response body could not be deserialized |
| `HubspotError::Hubspot(String)` | HubSpot returned a structured error |

---

## Dynamic Object Dispatch

If you need to select the object type at runtime, use `get_collection()`:

```rust
use hubspot::{ObjectType, types::OptionNotDesired};

let object_type = ObjectType::Contacts;
let collection = hubspot.objects.get_collection(object_type);
let results = collection
    .list::<MyProps, OptionNotDesired, OptionNotDesired>(Some(10), None, Some(false))
    .await?;
```

---

## HubSpot API Reference

This crate wraps the HubSpot CRM REST API (a mix of v3 and v4 endpoints). Useful reference docs:

- [Understanding the CRM](https://developers.hubspot.com/docs/guides/crm/understanding-the-crm) — objects, records, properties, and associations model
- [CRM Objects (v3)](https://developers.hubspot.com/docs/reference/api/crm/objects/object-types) — CRUD, batch, list operations
- [Associations (v4)](https://developers.hubspot.com/docs/guides/api/crm/associations) — association type IDs and management
- [Owners (v3)](https://developers.hubspot.com/docs/reference/api/crm/owners) — owner lookup
- [Private Apps](https://developers.hubspot.com/docs/api/private-apps) — generating the Bearer token for authentication
