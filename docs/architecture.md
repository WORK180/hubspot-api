# Architecture — hubspot-api

This document describes the internal design of the `hubspot` crate for contributors and AI agents working on the codebase.

---

## Design Goals

1. **Typed, zero-cost generics** — Callers define their own `Deserialize` structs for HubSpot properties. The crate uses generic type parameters rather than `HashMap<String, serde_json::Value>`, so unused fields cost nothing at runtime.
2. **No OpenSSL** — TLS via `rustls` only. No system library dependency, no OpenSSL build pain.
3. **Private App token auth** — A single Bearer token in every request header. No OAuth dance.
4. **One HTTP client** — A single `reqwest::Client` instance, shared via `Arc`, across the entire API surface.
5. **Automatic property query building** — Callers never enumerate HubSpot field names manually; the crate derives them from the struct's `Deserialize` field names at compile time.
6. **Extensible without forking** — Adding a new CRM object type or engagement type requires only adding an enum variant and wiring it into the manager — no changes to the generic core.

---

## Module Dependency Flow

```
src/lib.rs
│
├── src/builder.rs          (only depends on client/)
├── src/objects.rs          ──┐
├── src/engagements/mod.rs  ──┤──► src/api_configs/  ──► src/client/
├── src/engagements/notes.rs─┤
└── src/owners.rs           ──┘
```

**Hard rule:** `src/api_configs/` must never import from `src/objects/`, `src/engagements/`, or `src/owners/`. It is the generic infrastructure layer. Only `src/lib.rs` coordinates across all top-level modules.

---

## Core Abstractions

### `ToPath` Trait

```rust
pub trait ToPath {
    fn to_path(&self) -> String;
}
```

The single method maps an enum variant to the URL path segment HubSpot uses for that object type (e.g. `ObjectType::Contacts → "contacts"`, `EngagementType::Notes → "notes"`). Both `ObjectType` and `EngagementType` implement this trait.

---

### `ObjectApi<T: ToPath>` Trait

```rust
pub trait ObjectApi<T: ToPath> {
    fn name(&self) -> &T;
    fn path(&self) -> String;   // default: self.name().to_path()
    fn client(&self) -> &Arc<HubspotClient>;
}
```

Implemented by `ApiCollection<T>`, `AssociationsApiCollection<T>`, and `BatchApiCollection<T>`. Provides the common plumbing (object name, URL path, HTTP client reference) to every sub-collection.

---

### `ApiCollection<T: ToPath>`

The primary API surface for any object or engagement type:

```
ApiCollection<T>
├── list<P,PWH,A>(limit: Option<i32>, after: Option<&str>, archived: Option<bool>) -> HubspotResult<ListResult<HubspotRecord<P,PWH,A>>>
├── create<P,PWH,A>(HubspotRecord<P, OptionNotDesired, Vec<CreateAssociation>>) -> HubspotResult<HubspotRecord<P,PWH,A>>
├── read<P,PWH,A>(id: &str, archived: bool) -> HubspotResult<HubspotRecord<P,PWH,A>>
├── update<P,PWH>(id: String, properties: P) -> HubspotResult<HubspotRecord<P,PWH,OptionNotDesired>>
├── archive(id: String) -> HubspotResult<()>
├── associations: AssociationsApiCollection<T>
└── batch: BatchApiCollection<T>
```

`ApiCollection` instances are owned by the object/engagement manager structs. Callers navigate to them as struct fields:

```rust
hubspot.objects.deals.list(...)
hubspot.objects.deals.batch.read(...)
hubspot.objects.deals.associations.list(...)
hubspot.engagements.notes.create(...)
```

---

### `HubspotRecord<P, PWH, A>` — Tri-Parametric Generic Record

```rust
pub struct HubspotRecord<P, PWH, A> {
    pub id: String,
    pub properties: P,
    pub properties_with_history: PWH,
    pub associations: A,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    pub archived_at: Option<String>,
}
```

Type parameters:
- `P` — the properties the caller wants (their `Deserialize` struct)
- `PWH` — properties-with-history, or `OptionNotDesired`
- `A` — associations, or `OptionNotDesired`

`OptionNotDesired` is a unit struct that deserializes to nothing, making unused type slots truly zero-cost.

**Factory methods** (used to build request payloads):
- `HubspotRecord::with_properties(props)` — for PATCH/update payloads
- `HubspotRecord::with_properties_and_associations(props)` — for POST/create payloads with embedded associations
- `.attach_built_in_associations(link, ids)` — appends hardcoded `AssociationLinks` associations
- `.attach_associations(assoc_type, ids)` — appends custom association type IDs

---

### `serde_introspect` — Automatic Property Query Building

HubSpot's API requires explicitly listing desired properties in the query string:
`GET /crm/v3/objects/deals/123?properties=amount,dealname,closedate`

Rather than requiring callers to maintain that list, `ApiCollection` calls:

```rust
serde_aux::serde_introspection::serde_introspect::<P>()
```

This returns the field names of the `Deserialize` impl for type `P` at compile time. The crate then assembles the `?properties=` query string automatically. The caller only needs their struct:

```rust
#[derive(Deserialize)]
struct DealProperties {
    amount: String,
    dealname: String,
    closedate: String,
}
// No manual field listing needed — the crate handles it.
```

The same mechanism handles `propertiesWithHistory` if `PWH != OptionNotDesired`.

---

### `AssociationLinks` — Hardcoded Built-in Type IDs

HubSpot assigns numeric IDs to standard ("built-in") association types. These are encoded in the `AssociationLinks` plain enum to eliminate magic numbers in calling code:

```rust
pub enum AssociationLinks {
    NoteToContact,
    NoteToCompany,
    NoteToDeal,
}

impl AssociationLinks {
    pub fn build(&self) -> AssociationType { ... }
}
```

The numeric IDs (`202`, `190`, `214`) are produced by `.build()`, which returns an `AssociationType` with the `id` string and `"HUBSPOT_DEFINED"` category. Add new variants here as new built-in types are needed and wire them in `build()`. Do not scatter raw numeric IDs elsewhere in the codebase.

---

### `Arc<HubspotClient>` — Shared HTTP Layer

`Hubspot::new()` wraps the `HubspotClient` in `Arc`. Every manager (`ObjectsManager`, `EngagementsManager`, `OwnerApi`) and every `ApiCollection`, `AssociationsApiCollection`, and `BatchApiCollection` holds a clone of this Arc. There is exactly one `reqwest::Client` instance for the lifetime of the `Hubspot` struct.

`HubspotClient` is private to the crate. It exposes two internal methods:
- `begin(method, path)` — starts a `reqwest::RequestBuilder` with the base URL (`https://{domain}/{path}`); does **not** attach auth
- `send::<R: DeserializeOwned>(request)` — attaches `Authorization: Bearer {token}`, sends the request, and deserializes the response; maps HTTP/JSON errors to `HubspotError`

---

## Extension Points

### New CRM Object Type

Requires edits in `src/objects.rs` only:

1. New variant in `ObjectType` enum
2. New match arm in `ToPath` impl
3. New `ApiCollection<ObjectType>` field in `ObjectsManager`
4. Wire in `ObjectsManager::new()` and `ObjectsManager::get_collection()`

No changes to `api_configs/` needed.

### New Engagement Type

Requires edits in `src/engagements/`:

1. New variant in `EngagementType` enum
2. New match arm in `ToPath` impl
3. New `src/engagements/<type>.rs` with a `<Type>Properties` struct
4. New `ApiCollection<EngagementType>` field in `EngagementsManager`
5. Wire in `EngagementsManager::new()`

### New Built-in Association Type

Add a new variant to `AssociationLinks` in `src/api_configs/types.rs`, then update `AssociationLinks::build()` to map that variant to the correct HubSpot numeric type ID string.

---

## URL Structure

All requests target `https://{domain}/`. The crate uses a **mix of CRM API v3 and v4** endpoints:

| Operation | Endpoint |
|---|---|
| `list` | `crm/v3/objects/{to_path()}` |
| `read` | `crm/v3/objects/{to_path()}/{id}` |
| `update` | `crm/v3/objects/{to_path()}/{id}` |
| `archive` | `crm/v3/objects/{to_path()}/{id}` |
| `create` (single) | `crm/v4/objects/{to_path()}` |
| `batch/archive` | `crm/v3/objects/{to_path()}/batch/archive` |
| `batch/read` | `crm/v3/objects/{to_path()}/batch/read` |
| `batch/update` | `crm/v3/objects/{to_path()}/batch/update` |
| `batch/create` | `crm/v4/objects/{to_path()}` |
| associations `list` | `crm/v4/objects/{to_path()}/{id}/associations/{to_type}` |
| associations `create` | `crm/v4/objects/{to_path()}/{id}/associations/{to_type}/{to_id}` |
| associations `delete` | `crm/v4/objects/{to_path()}/{id}/associations/{to_type}/{to_id}` |
| owners `read` | `crm/v3/owners/{id}` |

Query strings are assembled in `src/api_configs/query.rs` using `build_query_string()` and `build_paging_query()`.

---

## HubSpot API Reference

This crate is built against the HubSpot CRM REST API. Key reference docs:

- [Understanding the CRM](https://developers.hubspot.com/docs/guides/crm/understanding-the-crm) — objects, records, properties, and associations model
- [CRM Objects (v3)](https://developers.hubspot.com/docs/reference/api/crm/objects/object-types) — list, read, update, archive, batch operations
- [CRM Objects — Create (v4)](https://developers.hubspot.com/docs/reference/api/crm/objects) — v4 create endpoint used by this crate
- [Associations (v4)](https://developers.hubspot.com/docs/guides/api/crm/associations) — association type IDs and create/delete endpoints
- [Owners (v3)](https://developers.hubspot.com/docs/reference/api/crm/owners) — owner lookup
- [Private Apps](https://developers.hubspot.com/docs/api/private-apps) — how to generate the Bearer token used for authentication
