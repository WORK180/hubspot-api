# Copilot Workspace Instructions — hubspot-api

See [CLAUDE.md](../CLAUDE.md) for the full workspace instructions, architecture overview, module map, conventions, and extension guide.

<!--
- **Engagements** — `Notes` (extensible to tasks, meetings, calls)
- **Owners** — read HubSpot users by ID

---

## Module Map

```
hubspot (src/lib.rs)                 ← Hubspot struct, public re-exports
├── builder (src/builder.rs)         ← HubspotBuilder, HubspotBuilderError
├── client/
│   ├── mod.rs                       ← HubspotClient (private, Arc-shared)
│   └── error.rs                     ← HubspotResult<T>, HubspotError
├── api_configs/                     ← Core generic API layer (do not couple to object types)
│   ├── mod.rs                       ← ApiCollection<T>: list/create/read/update/archive
│   ├── types.rs                     ← HubspotRecord<P,PWH,A>, ToPath, ObjectApi<T>, OptionNotDesired
│   ├── associations.rs              ← AssociationsApiCollection<T>
│   ├── batch.rs                     ← BatchApiCollection<T>, BatchResult
│   └── query.rs                     ← URL query string builders
├── objects (src/objects.rs)         ← ObjectType enum, ObjectsManager
├── engagements/
│   ├── mod.rs                       ← EngagementType enum, EngagementsManager
│   └── notes.rs                     ← NoteProperties
└── owners (src/owners.rs)           ← Owner, Team, OwnerApi
```

Dependency flow: `lib.rs` → `objects`/`engagements`/`owners` → `api_configs` → `client`.
`api_configs` must **never** import from `objects` or `engagements` — it is the generic core.

---

## Key Patterns — follow these when extending the crate

### 1. Builder Pattern

`Hubspot::builder()` returns `HubspotBuilder`. All required fields (`domain`, `token`, `portal_id`) are validated at `build()`. Callers may inject a custom `reqwest::Client` via `.client()`.

### 2. Arc-Shared Client

`HubspotClient` is wrapped in `Arc<HubspotClient>` in `Hubspot::new()`. Every sub-manager receives a clone of the Arc. There is exactly **one** underlying HTTP client instance.

### 3. `HubspotRecord<P, PWH, A>` — Tri-Parametric Generic Record

- `P` = Properties type (caller-defined `Deserialize` struct)
- `PWH` = PropertiesWithHistory type (or `OptionNotDesired`)
- `A` = Associations type (or `OptionNotDesired`)

Use `OptionNotDesired {}` as a zero-cost sentinel for type parameters the caller doesn't need.

### 4. `serde_introspect` — Automatic Property Query Building

`serde_aux::serde_introspection::serde_introspect::<P>()` extracts field names from a caller's `Deserialize` struct at compile time to build `?properties=field1,field2` automatically. Callers never enumerate field names manually.

### 5. `ToPath` + `ObjectApi<T>` Trait System

- `ToPath` — single method `to_path(&self) -> String`, maps an enum variant to a URL path segment
- `ObjectApi<T: ToPath>` — provides `name()`, `path()`, `client()` to `ApiCollection<T>`
- `ApiCollection<T>` is generic over any `T: ToPath`; the same CRUD/batch/association code serves all object types

### 6. Adding a New Object Type

1. Add a variant to `ObjectType` enum in `src/objects.rs` (strum `Display` handles the string)
2. Implement `ToPath` for the new variant (lowercase, plural, e.g. `"line_items"`)
3. Add an `ApiCollection<ObjectType>` field to `ObjectsManager`
4. Wire it in `ObjectsManager::new()` and `ObjectsManager::get_collection()`

### 7. Adding a New Engagement Type

1. Add a variant to `EngagementType` in `src/engagements/mod.rs`
2. Implement `ToPath` for the new variant
3. Create `src/engagements/<type>.rs` with a `<Type>Properties` struct (see `notes.rs` as reference)
4. Add an `ApiCollection<EngagementType>` field to `EngagementsManager`

### 8. Association Type IDs

`AssociationLinks` enum in `src/api_configs/types.rs` encodes known HubSpot built-in association type IDs as constants. Add new variants here for new built-in association types; do **not** scatter magic numbers in calling code.

---

## Conventions

- **TLS**: Always `rustls-tls`. Do not enable `native-tls` or `openssl`.
- **Time**: Use `time::OffsetDateTime` with `#[serde(with = "time::serde::rfc3339")]` for timestamps.
- **Enum display**: Use `strum_macros::Display` on enum variants that map to URL path strings. Note: `ObjectType::LineItems` requires an explicit `to_path()` match arm rather than relying on the default lowercase display.
- **Error type**: Return `HubspotResult<T>` (alias for `Result<T, HubspotError>`) from all public async methods.
- **Field naming**: HubSpot API uses camelCase query params (`propertiesWithHistory`) and snake_case JSON fields (mapped via `#[serde(rename = "...")]`).

---

## Known Issues

- **README mismatch**: The README shows `.key("token")` but the actual builder method is `.token("...")`. Use `.token()` in all new code and documentation.
- **`ObjectType::ToPath` TODO**: `src/objects.rs` has a comment `// TODO see if we can use strum` — strum's `to_string()` would work but the current explicit match is fine.

---

## Reference Docs

- [Architecture & design](../docs/architecture.md)
- [Consumer API reference](../docs/api.md)
- [Module: api_configs](../docs/modules/api_configs.md)
- [Module: objects](../docs/modules/objects.md)
- [Module: engagements](../docs/modules/engagements.md)
- [Module: owners](../docs/modules/owners.md)
- [Module: client & errors](../docs/modules/client.md)
-->
