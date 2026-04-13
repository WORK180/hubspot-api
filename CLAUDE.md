# CLAUDE.md — hubspot-api

Rust crate `hubspot` v0.2.5 — an unofficial async client for the HubSpot CRM REST API.
Authentication: Private App tokens (Bearer). TLS: rustls only (no OpenSSL).

---

## Build & Development Commands

```sh
# Build
cargo build

# Run all tests (requires .env with HUBSPOT_TOKEN, HUBSPOT_DOMAIN, HUBSPOT_PORTAL_ID)
cargo test

# Lint & style
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Format in place
cargo fmt

# Package (dry run)
cargo package --no-verify
```

---

## Architecture Overview

The crate is built around a generic `ApiCollection<T: ToPath>` that provides list/create/read/update/archive/batch/associations for any HubSpot object type. Concrete object types (`Contacts`, `Companies`, `Deals`, `LineItems`) and engagement types (`Notes`) implement `ToPath` and are wired into named manager structs (`ObjectsManager`, `EngagementsManager`). A single Arc-shared `HubspotClient` handles all HTTP calls. Callers define their own `Deserialize` structs for typed properties; the crate uses `serde_introspect` to automatically build the `?properties=` query string from those structs' field names.

---

## Module Map

```
hubspot (src/lib.rs)                 ← Hubspot struct, public re-exports
├── builder (src/builder.rs)         ← HubspotBuilder, HubspotBuilderError
├── client/
│   ├── mod.rs                       ← HubspotClient (private, Arc-shared HTTP layer)
│   └── error.rs                     ← HubspotResult<T>, HubspotError
├── api_configs/                     ← Generic API infrastructure — no coupling to object types
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

**Dependency rule:** `api_configs` must never import from `objects`, `engagements`, or `owners`. It is the generic core. Only `lib.rs` coordinates across all top-level modules.

---

## Core Abstractions

### `HubspotRecord<P, PWH, A>`
Tri-parametric generic record. Type parameters:
- `P` — Properties (caller-defined `Deserialize` struct for the fields they need)
- `PWH` — PropertiesWithHistory (or `OptionNotDesired {}` sentinel)
- `A` — Associations (or `OptionNotDesired {}` sentinel)

`OptionNotDesired` is a unit struct that implements `Serialize + Deserialize + Default`. It deserializes to `None` / an empty value without allocating, making unused type parameters truly zero-cost.

### `serde_introspect` Auto-Query Building
```rust
serde_aux::serde_introspection::serde_introspect::<MyProperties>()
```
Returns field names from the `Deserialize` impl at compile time. `ApiCollection` calls this automatically to build `?properties=name,amount,...`. Callers never list field names manually.

### `ToPath` + `ObjectApi<T>`
```rust
pub trait ToPath { fn to_path(&self) -> String; }
pub trait ObjectApi<T: ToPath> { fn name(&self) -> &str; fn path(&self) -> String; fn client(&self) -> &Arc<HubspotClient>; }
```
`ApiCollection<T>` is generic over any `T: ToPath`. The same CRUD/batch/associations implementation serves every object and engagement type with no duplication.

### `AssociationLinks` Enum
Encodes HubSpot's built-in association type IDs as variants to eliminate magic numbers:
- `NoteToContact = 202`
- `NoteToCompany = 190`
- `NoteToDeal = 214`

---

## Extension Guide

### Adding a New CRM Object Type

1. **Add enum variant** in `src/objects.rs`:
   ```rust
   pub enum ObjectType {
       Contacts,
       Companies,
       Deals,
       LineItems,
       Tickets,  // ← new
   }
   ```

2. **Implement `ToPath`** in `src/objects.rs` (add a match arm):
   ```rust
   ObjectType::Tickets => "tickets".to_string(),
   ```

3. **Add field to `ObjectsManager`** in `src/objects.rs`:
   ```rust
   pub tickets: ApiCollection<ObjectType>,
   ```

4. **Wire in `ObjectsManager::new()`**:
   ```rust
   tickets: ApiCollection::new(ObjectType::Tickets, client.clone()),
   ```

5. **Wire in `ObjectsManager::get_collection()`**:
   ```rust
   ObjectType::Tickets => &self.tickets,
   ```

### Adding a New Engagement Type

1. Add a variant to `EngagementType` in `src/engagements/mod.rs`
2. Implement `ToPath` for the new variant
3. Create `src/engagements/<type>.rs` with a `<Type>Properties` struct:
   ```rust
   #[derive(Debug, Deserialize, Serialize)]
   pub struct TaskProperties {
       #[serde(rename = "hs_task_subject")]
       pub subject: String,
   }
   ```
4. Add an `ApiCollection<EngagementType>` field to `EngagementsManager`
5. Wire it in `EngagementsManager::new()`

### Adding a New Built-in Association Type

Add a variant to `AssociationLinks` in `src/api_configs/types.rs` with the HubSpot numeric type ID as the discriminant value.

---

## Conventions

| Topic | Convention |
|---|---|
| TLS | Always `rustls-tls`. Never enable `native-tls` or link OpenSSL. |
| Timestamps | `time::OffsetDateTime` with `#[serde(with = "time::serde::rfc3339")]` |
| Enum → URL path | `strum_macros::Display` + `#[strum(serialize_all = "snake_case")]` |
| Return type | `HubspotResult<T>` (alias: `Result<T, HubspotError>`) from all public async methods |
| HubSpot JSON fields | Mapped via `#[serde(rename = "hs_field_name")]` on struct fields |
| HTTP client | Inject via `Arc<HubspotClient>::clone()`, never construct a new client in a nested struct |
| Builder method | Use `.token("...")` — **not** `.key("...")`. The README has a bug; the actual method is `.token()`. |

---

## Known Issues / TODOs

| Location | Issue |
|---|---|
| `README.md` | Shows `.key("token")` but the builder method is `.token("...")` — fix before v0.3 |
| `src/objects.rs` | `// TODO see if we can use strum` on `ToPath` impl — strum's `to_string()` could replace the explicit match |
| `Cargo.toml` | `async-trait = "0.1"` is listed but not currently used in any source file |

---

## Reference Docs

- [docs/architecture.md](docs/architecture.md) — deep-dive on design decisions and abstractions
- [docs/api.md](docs/api.md) — consumer quick-start and full API reference
- [docs/modules/api_configs.md](docs/modules/api_configs.md)
- [docs/modules/objects.md](docs/modules/objects.md)
- [docs/modules/engagements.md](docs/modules/engagements.md)
- [docs/modules/owners.md](docs/modules/owners.md)
- [docs/modules/client.md](docs/modules/client.md)
