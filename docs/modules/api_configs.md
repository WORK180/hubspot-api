# Module: api_configs

Sources: `src/api_configs/mod.rs`, `src/api_configs/types.rs`, `src/api_configs/associations.rs`, `src/api_configs/batch.rs`, `src/api_configs/query.rs`

The generic API infrastructure layer. **Must never import from `objects`, `engagements`, or `owners`.** Contains all CRUD, batch, association, and query-building logic in a form generic over any `T: ToPath`.

---

## `ApiCollection<T: ToPath>`

Source: `src/api_configs/mod.rs`

The main per-object API surface. One instance exists per object or engagement type, owned by the respective manager struct.

```rust
pub struct ApiCollection<T: ToPath> {
    pub associations: AssociationsApiCollection<T>,
    pub batch: BatchApiCollection<T>,
    // ... private fields
}
```

### Methods

#### `list`
```rust
pub async fn list<P, PWH, A>(
    &self,
    limit: Option<u32>,
    after: Option<String>,
    archived: bool,
) -> HubspotResult<ListResult<HubspotRecord<P, PWH, A>>>
where
    P: DeserializeOwned + Introspect,
    PWH: DeserializeOwned + Introspect,
    A: DeserializeOwned,
```

Lists records with optional pagination. Property names are derived automatically from `P` via `serde_introspect`.

#### `create`
```rust
pub async fn create<P, PWH, A>(
    &self,
    record: HubspotRecord<P, PWH, A>,
) -> HubspotResult<HubspotRecord<P, PWH, A>>
```

Creates a record. Use `HubspotRecord::with_properties()` or `::with_properties_and_associations()` to build the payload.

#### `read`
```rust
pub async fn read<P, PWH, A>(
    &self,
    id: &str,
    archived: bool,
) -> HubspotResult<HubspotRecord<P, PWH, A>>
```

Fetches a single record by ID.

#### `update`
```rust
pub async fn update<P, PWH>(
    &self,
    id: &str,
    record: HubspotRecord<P, PWH, OptionNotDesired>,
) -> HubspotResult<HubspotRecord<P, PWH, OptionNotDesired>>
```

Partially updates a record (PATCH). Properties not included in the payload are left unchanged.

#### `archive`
```rust
pub async fn archive(&self, id: &str) -> HubspotResult<()>
```

Soft-deletes a record (sets `archived: true`).

---

## `AssociationsApiCollection<T: ToPath>`

Source: `src/api_configs/associations.rs`

Manages associations between two HubSpot objects. Accessible as `.associations` on any `ApiCollection`.

### `list`
```rust
pub async fn list(
    &self,
    from_id: &str,
    to_object_type: impl ToPath,
) -> HubspotResult<ListResult<Association>>
```

Lists all associations from a given object to objects of another type.

### `create`
```rust
pub async fn create(
    &self,
    from_id: &str,
    to_id: &str,
    association: impl Into<AssociationCreationDetails>,
) -> HubspotResult<CreatedAssociationResult>
```

Creates a single association. Accepts either `AssociationLinks` (built-in types) or `AssociationCreationDetails` (custom types).

### `delete`
```rust
pub async fn delete(
    &self,
    from_id: &str,
    to_object_type: impl ToPath,
    to_id: &str,
) -> HubspotResult<()>
```

Removes all associations between two specific records.

### Associated Types

```rust
pub struct Association {
    pub to_object_id: i64,
    pub association_types: Vec<AssociationTypes>,
}

pub struct AssociationTypes {
    pub category: String,
    pub type_id: i64,
    pub label: Option<String>,
}

pub struct AssociationCreationDetails {
    pub category: String,
    pub type_id: i64,
}

pub struct CreatedAssociationResult {
    pub from_object_type_id: String,
    pub from_object_id: i64,
    pub to_object_id: i64,
    pub labels: Vec<String>,
}
```

---

## `BatchApiCollection<T: ToPath>`

Source: `src/api_configs/batch.rs`

Bulk operations for performance-sensitive workflows. Accessible as `.batch` on any `ApiCollection`.

### `read`
Reads a batch of records by ID.

> **Note:** The exact `read` signature in `BatchApiCollection` is defined in `src/api_configs/batch.rs`. The previous signature shown here was out of sync with the implementation, so this section intentionally avoids restating incorrect parameter and return types.

### `create`
Creates records in batch.

> **Note:** The exact `create` signature in `BatchApiCollection` is defined in `src/api_configs/batch.rs`. The previous signature shown here was out of sync with the implementation, so this section intentionally avoids restating incorrect parameter and return types.

### `update`
Updates records in batch.

> **Note:** The exact `update` signature in `BatchApiCollection` is defined in `src/api_configs/batch.rs`. The previous signature shown here was out of sync with the implementation, so this section intentionally avoids restating incorrect parameter and return types.

### `archive`
```rust
pub async fn archive(&self, ids: Vec<&str>) -> HubspotResult<()>
```

### `BatchResult<P, PWH, A>`

```rust
pub struct BatchResult<P, PWH, A> {
    pub status: String,
    pub results: Vec<HubspotRecord<P, PWH, A>>,
    pub requested_at: Option<OffsetDateTime>,
    pub started_at: Option<OffsetDateTime>,
    pub completed_at: Option<OffsetDateTime>,
    pub links: HashMap<String, String>,
}
```

---

## `HubspotRecord<P, PWH, A>`

Source: `src/api_configs/types.rs`

Primary response and request type. See [docs/architecture.md](../architecture.md#hubspotrecordp-pwh-a--tri-parametric-generic-record) for design rationale. In the current implementation, `properties_with_history` and `associations` are generic fields populated via `#[serde(default)]`, and HubSpot timestamps are represented as optional raw strings.

}
```

### Factory Methods

```rust
// For update (PATCH) payloads
HubspotRecord::with_properties(props: P) -> Self

// For create (POST) payloads with embedded associations
HubspotRecord::with_properties_and_associations(props: P) -> Self
```

### Association Attachment

```rust
// Attach a known built-in HubSpot association type
record.attach_built_in_associations(link: AssociationLinks, ids: Vec<String>)

// Attach a custom association type
record.attach_associations(assoc_type: AssociationType, ids: Vec<String>)
```

---

## `OptionNotDesired`

```rust
pub struct OptionNotDesired {}
```

A zero-cost sentinel unit struct. Use as a type argument for `PWH` or `A` when you don't need that data. Implements `Serialize + Deserialize + Default`.

---

## `AssociationLinks`

```rust
pub enum AssociationLinks {
    NoteToContact = 202,
    NoteToCompany = 190,
    NoteToDeal    = 214,
}
```

Encodes HubSpot's built-in association type numeric IDs. Add new variants here when HubSpot introduces new built-in types or the crate gains support for new engagement types.

---

## `ListResult<T>`

```rust
pub struct ListResult<T> {
    pub results: Vec<T>,
    pub paging: Option<Paging>,
}

pub struct Paging {
    pub next: PagingNext,
}

pub struct PagingNext {
    pub after: String,
    pub link: String,
}
```

Pass `paging.next.after` as the `after` argument to the next `list()` call for cursor-based pagination.

---

## `query.rs` — URL Query String Helpers

Internal helpers used by `ApiCollection`, not part of the public API.

| Function | Purpose |
|---|---|
| `query_begun_check(started: bool)` | Returns `"?"` or `"&"` based on whether query has started |
| `build_paging_query(limit, after)` | Builds `?limit=N&after=X` |
| `build_query_string(properties, pwh_properties, associations, archived)` | Assembles the full query string with `properties=`, `propertiesWithHistory=`, `associations=`, `archived=` |

---

## Traits

### `ToPath`

```rust
pub trait ToPath {
    fn to_path(&self) -> String;
}
```

Implemented by `ObjectType` and `EngagementType`. Maps an enum variant to its HubSpot URL path segment.

### `ObjectApi<T: ToPath>`

```rust
pub trait ObjectApi<T: ToPath> {
    fn name(&self) -> &str;
    fn path(&self) -> String;
    fn client(&self) -> &Arc<HubspotClient>;
}
```

Implemented by `ApiCollection<T>`, `AssociationsApiCollection<T>`, and `BatchApiCollection<T>`. Provides shared plumbing for path construction and HTTP client access.
