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
    .client(client)
    .build()?;
```

---

## Defining Properties

Define a struct for the HubSpot properties you want returned. Field names must match the [serde field names](https://serde.rs/field-attrs.html); the crate automatically maps them to `?properties=` query parameters.

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

HubSpot property names (like `dealname`) go in `#[serde(rename = "...")]`. Your Rust field names can be whatever you prefer.

---

## CRM Objects

Available object types: `Contacts`, `Companies`, `Deals`, `LineItems`.

```rust
use hubspot::ObjectType;
```

### Read (Get by ID)

```rust
use hubspot::types::{HubspotRecord, OptionNotDesired};

let deal: HubspotRecord<DealProperties, OptionNotDesired, OptionNotDesired> =
    hubspot.objects.deals.read("123", false).await?;

println!("{}", deal.properties.name);
```

### List

```rust
let results = hubspot.objects.deals
    .list::<DealProperties, OptionNotDesired, OptionNotDesired>(
        Some(10),   // limit
        None,       // after cursor (pagination)
        false,      // include archived
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

```rust
use hubspot::types::{HubspotRecord, OptionNotDesired};

let payload = HubspotRecord::<DealProperties, OptionNotDesired, OptionNotDesired>
    ::with_properties(DealProperties {
        name: "New Deal".to_string(),
        amount: Some("5000".to_string()),
        close_date: None,
    });

let created = hubspot.objects.deals.create(payload).await?;
println!("Created deal ID: {}", created.id);
```

### Update

```rust
let payload = HubspotRecord::<DealProperties, OptionNotDesired, OptionNotDesired>
    ::with_properties(DealProperties {
        name: "Updated Deal Name".to_string(),
        amount: None,
        close_date: None,
    });

let updated = hubspot.objects.deals.update("123", payload).await?;
```

### Archive (Soft Delete)

```rust
hubspot.objects.deals.archive("123").await?;
```

---

## Batch Operations

### Batch Read

```rust
let ids = vec!["123".to_string(), "456".to_string(), "789".to_string()];

let batch = hubspot.objects.deals
    .batch
    .read::<DealProperties, OptionNotDesired, OptionNotDesired>(ids)
    .await?;

for deal in batch.results {
    println!("{}: {}", deal.id, deal.properties.name);
}
```

### Batch Create

```rust
use hubspot::types::HubspotRecord;

let records = vec![
    HubspotRecord::with_properties(DealProperties { name: "Deal A".into(), .. }),
    HubspotRecord::with_properties(DealProperties { name: "Deal B".into(), .. }),
];

let result = hubspot.objects.deals.batch.create(records).await?;
```

### Batch Update

```rust
let updates = vec![("123".to_string(), DealProperties { name: "Updated A".into(), .. })];
let result = hubspot.objects.deals.batch.update(updates).await?;
```

### Batch Archive

```rust
hubspot.objects.deals.batch.archive(vec!["123", "456"]).await?;
```

---

## Associations

### List Associations

```rust
use hubspot::ObjectType;

// List all contacts associated with deal 123
let associations = hubspot.objects.deals
    .associations
    .list("123", ObjectType::Contacts)
    .await?;

for assoc in associations.results {
    println!("Contact ID: {}, type: {}", assoc.to_object_id, assoc.association_types[0].type_id);
}
```

### Create Association (Built-in Type)

Use `AssociationLinks` for HubSpot's built-in association types:

```rust
use hubspot::types::AssociationLinks;

// Associate note 789 with contact 456
hubspot.engagements.notes
    .associations
    .create("789", "456", AssociationLinks::NoteToContact)
    .await?;
```

### Create Association (Custom Type)

```rust
use hubspot::associations::AssociationCreationDetails;

hubspot.objects.deals
    .associations
    .create("123", "456", AssociationCreationDetails {
        category: "USER_DEFINED".to_string(),
        type_id: 99,
    })
    .await?;
```

### Delete Association

```rust
hubspot.objects.deals
    .associations
    .delete("123", ObjectType::Contacts, "456")
    .await?;
```

---

## Engagements — Notes

```rust
use hubspot::notes::NoteProperties;
use hubspot::types::{HubspotRecord, OptionNotDesired, AssociationLinks};

// Build a note associated with a contact and a deal
let mut note =
    HubspotRecord::with_properties_and_associations(NoteProperties::new("Call went well.".to_string()));

note.attach_built_in_associations(AssociationLinks::NoteToContact, vec!["456".to_string()]);
note.attach_built_in_associations(AssociationLinks::NoteToDeal, vec!["123".to_string()]);

let created = hubspot.engagements.notes.create(note).await?;
println!("Note ID: {}", created.id);
```

`NoteProperties::new(body)` automatically sets `hs_timestamp` to the current time.

---

## Owners

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

When you need the change history of a property, provide a type for the `PWH` parameter:

```rust
#[derive(Debug, Deserialize)]
struct DealHistory {
    amount: Option<Vec<serde_json::Value>>,
}

let deal = hubspot.objects.deals
    .read::<DealProperties, DealHistory, OptionNotDesired>("123", false)
    .await?;

println!("{:?}", deal.properties_with_history.amount);
```

---

## Reading with Associations Inline

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

```rust
match hubspot.objects.deals.read::<DealProperties, _, _>("bad-id", false).await {
    Ok(deal) => println!("{}", deal.properties.name),
    Err(e) => eprintln!("Request failed: {e}"),
}
```

| Variant | When |
|---|---|
| `HubspotError::Http(reqwest::Error)` | Network failure, timeout, non-success HTTP status |
| `HubspotError::Json(serde_json::Error)` | Response body could not be deserialized |
| `HubspotError::Hubspot(String)` | HubSpot returned a structured error (message extracted from `HubspotErrorResponse`) |

---

## Dynamic Object Dispatch

If you need to select the object type at runtime, use `get_collection()`:

```rust
use hubspot::ObjectType;

let object_type = ObjectType::Contacts;
let collection = hubspot.objects.get_collection(object_type);
let results = collection.list::<MyProps, _, _>(Some(10), None, Some(false)).await?;
```

---

## HubSpot API Reference

This crate wraps the HubSpot CRM REST API (a mix of v3 and v4 endpoints). Useful reference docs:

- [Understanding the CRM](https://developers.hubspot.com/docs/guides/crm/understanding-the-crm) — objects, records, properties, and associations model
- [CRM Objects (v3)](https://developers.hubspot.com/docs/reference/api/crm/objects/object-types) — CRUD, batch, list operations
- [Associations (v4)](https://developers.hubspot.com/docs/guides/api/crm/associations) — association type IDs and management
- [Owners (v3)](https://developers.hubspot.com/docs/reference/api/crm/owners) — owner lookup
- [Private Apps](https://developers.hubspot.com/docs/api/private-apps) — generating the Bearer token for authentication
