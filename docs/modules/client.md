# Module: client

Sources: `src/client/mod.rs`, `src/client/error.rs`

The HTTP transport layer. `HubspotClient` is private to the crate — consumers never interact with it directly. All public async methods return `HubspotResult<T>`.

---

## `HubspotClient` (private)

```rust
pub(crate) struct HubspotClient {
    client: reqwest::Client,
    token: String,
    domain: String,
    portal_id: String,
}
```

Constructed once in `Hubspot::new()` and shared via `Arc<HubspotClient>` across all managers and collections. Uses `reqwest` with `rustls-tls` (no OpenSSL).

### Internal Methods

#### `begin`
```rust
pub(crate) fn begin(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder
```

Constructs a `RequestBuilder` with the base URL (`https://{domain}/{path}`). Callers pass the full path including version (e.g. `"crm/v3/objects/contacts"` or `"crm/v4/objects/deals"`). All public API methods start from `begin()`. Bearer authentication is **not** applied here — it is added by `send()`.

#### `send`
```rust
pub(crate) async fn send<R: DeserializeOwned>(&self, request: reqwest::RequestBuilder) -> HubspotResult<R>
```

Sends the request (attaching the `Authorization: Bearer {token}` header), checks for HTTP errors, and deserializes the response body into `R`. Maps failures to `HubspotError`.

---

## `HubspotResult<T>`

```rust
pub type HubspotResult<T> = Result<T, HubspotError>;
```

The return type for all public async methods in this crate.

---

## `HubspotError`

```rust
pub enum HubspotError {
    Json(serde_json::Error),
    Http(reqwest::Error),
    Hubspot(String),
}
```

Implements `std::error::Error` and `std::fmt::Display`.

| Variant | When it occurs |
|---|---|
| `Json(serde_json::Error)` | The response body could not be deserialized into the expected type |
| `Http(reqwest::Error)` | Network failure, connection timeout, or non-success HTTP status from `reqwest` |
| `Hubspot(String)` | HubSpot returned a structured error response; the `message` field is extracted and stored |

### Error Handling Example

```rust
match hubspot.objects.contacts.read::<MyProps, _, _>("bad-id", false).await {
    Ok(record) => { /* use record */ }
    Err(err) => eprintln!("HubSpot request failed: {err}"),
}
```

---

## `HubspotErrorResponse` (internal helper)

Used to deserialize HubSpot's structured error JSON before converting to `HubspotError::Hubspot`.

```rust
pub struct HubspotErrorResponse {
    pub message: String,
    pub context: HubspotErrorContext,
    pub category: String,
}

pub struct HubspotErrorContext {
    pub properties: Vec<String>,
}
```

These types are not part of the public-facing API surface. When HubSpot returns a 4xx/5xx with a JSON body matching this shape, `send()` converts it via `From<HubspotErrorResponse>` which formats the error as `"{category}: {message}, {context:?}"`.
