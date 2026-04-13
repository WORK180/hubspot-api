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

Constructs a `RequestBuilder` with the base URL (`https://{domain}/crm/v3/`) and the `Authorization: Bearer {token}` header pre-applied. All public API methods start from `begin()`.

#### `send`
```rust
pub(crate) async fn send<R: DeserializeOwned>(&self, request: reqwest::RequestBuilder) -> HubspotResult<R>
```

Sends the request, checks for HTTP errors, and deserializes the response body into `R`. Maps failures to `HubspotError`.

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
use hubspot::client::error::HubspotError;

match hubspot.objects.contacts.read::<MyProps, _, _>("bad-id", false).await {
    Ok(record) => { /* use record */ }
    Err(HubspotError::Hubspot(msg)) => eprintln!("HubSpot API error: {msg}"),
    Err(HubspotError::Http(err)) => eprintln!("HTTP error: {err}"),
    Err(HubspotError::Json(err)) => eprintln!("Deserialization error: {err}"),
}
```

---

## `HubspotErrorResponse` (internal)

Used to deserialize HubSpot's structured error JSON before converting to `HubspotError::Hubspot(message)`.

```rust
struct HubspotErrorResponse {
    pub message: String,
    pub context: HubspotErrorContext,
    pub category: String,
}

struct HubspotErrorContext {
    pub properties: Vec<String>,
}
```

These types are not part of the public API. When HubSpot returns a 4xx/5xx with a JSON body matching this shape, `send()` extracts `message` and returns `HubspotError::Hubspot(message)`.
