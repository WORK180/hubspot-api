# Module: owners

Source: `src/owners.rs`

Provides read access to HubSpot Owners (users who can be assigned to CRM records).

---

## `Owner`

```rust
pub struct Owner {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub user_id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub archived: bool,
    pub teams: Option<Vec<Team>>,
}
```

Timestamps use `time::OffsetDateTime` deserialized via `time::serde::rfc3339`.

---

## `Team`

```rust
pub struct Team {
    pub id: String,
    pub name: String,
    pub primary: bool,
}
```

`primary: true` indicates this is the owner's primary team. Owners may belong to multiple teams.

---

## `OwnerApi`

```rust
pub struct OwnerApi { /* private fields */ }
```

Owned by `Hubspot` as `hubspot.owners`.

### `read()`

```rust
pub async fn read(&self, id: &str, archived: Option<bool>) -> HubspotResult<Owner>
```

Fetches a single owner by their HubSpot owner ID.

- `id` — HubSpot owner ID (numeric string, e.g. `"12345678"`)
- `archived` — pass `Some(true)` to retrieve archived (deactivated) owners; `None` defaults to `false`

```rust
let owner = hubspot.owners.read("12345678", None).await?;
println!("{} {} <{}>", owner.first_name, owner.last_name, owner.email);

if let Some(teams) = owner.teams {
    for team in teams {
        println!("Team: {} (primary: {})", team.name, team.primary);
    }
}
```

---

## Notes

- There is no `list` endpoint implemented for owners in this crate. Only lookup by ID is supported.
- Owner IDs appear on CRM objects in properties like `hubspot_owner_id`. Pass that value to `owners.read()` to resolve the full owner record.
