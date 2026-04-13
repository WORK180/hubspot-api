# Module: objects

Source: `src/objects.rs`

Provides the `ObjectType` enum and `ObjectsManager` struct, which wire concrete CRM object types into the generic `ApiCollection<T>` infrastructure.

---

## `ObjectType`

```rust
pub enum ObjectType {
    Contacts,
    Companies,
    Deals,
    LineItems,
}
```

Implements:
- `ToPath` — maps each variant to its HubSpot URL path segment
- `Display` (via `strum_macros::Display`)
- `Clone`, `Debug`

### `ToPath` Mapping

| Variant | `to_path()` output |
|---|---|
| `Contacts` | `"contacts"` |
| `Companies` | `"companies"` |
| `Deals` | `"deals"` |
| `LineItems` | `"line_items"` |

> **TODO:** The implementation uses an explicit `match`. A future cleanup could use `strum`'s `to_string()` instead.

---

## `ObjectsManager`

```rust
pub struct ObjectsManager {
    pub contacts:   ApiCollection<ObjectType>,
    pub companies:  ApiCollection<ObjectType>,
    pub deals:      ApiCollection<ObjectType>,
    pub line_items: ApiCollection<ObjectType>,
}
```

Owned by `Hubspot`. Each field is an `ApiCollection<ObjectType>` pre-configured for that object type. All four expose the full CRUD + batch + associations surface:

```rust
hubspot.objects.contacts.list(...)
hubspot.objects.deals.read(...)
hubspot.objects.companies.batch.create(...)
hubspot.objects.line_items.associations.list(...)
```

### `get_collection()`

```rust
pub fn get_collection(&self, object_type: ObjectType) -> &ApiCollection<ObjectType>
```

Resolves an `ObjectType` variant to the corresponding `ApiCollection` reference at runtime. Useful when the object type is determined dynamically.

```rust
let collection = hubspot.objects.get_collection(ObjectType::Deals);
let results = collection.list::<MyProps, _, _>(Some(10), None, false).await?;
```

---

## Adding a New Object Type

See [docs/architecture.md](../architecture.md#new-crm-object-type) for the full step-by-step guide.

In brief — edits in `src/objects.rs` only:
1. Add variant to `ObjectType`
2. Add `to_path()` match arm
3. Add `ApiCollection<ObjectType>` field to `ObjectsManager`
4. Wire in `::new()` and `get_collection()`
