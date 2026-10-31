<a class="hidden" href="https://www.mailerlite.com"><img src="https://app.mailerlite.com/assets/images/logo-color.png" width="200px"/></a>

# MailerLite Rust SDK

![Build](https://img.shields.io/github/actions/workflow/status/mailerlite/mailerlite-rust/main.yaml)
![Downloads](https://img.shields.io/crates/d/mailerlite-rs?label=downloads)
![Crates.io](https://img.shields.io/crates/v/mailerlite-rs)
![License](https://img.shields.io/crates/l/mailerlite-rs)

A typed, asynchronous Rust client for the
[current MailerLite API](https://developers.mailerlite.com/getting-started).
Manage subscribers, campaigns, automations, and ecommerce through resource APIs
with typed requests, responses, pagination, and errors.

[API reference](https://docs.rs/mailerlite-rs) ·
[Examples](examples/README.md) ·
[Upgrade from v1](UPGRADE.md) ·
[Contributing](CONTRIBUTING.md)

## Installation

Requires **Rust 1.88 or newer** and a Tokio runtime. Add these dependencies to
your application's `Cargo.toml`:

```toml
[dependencies]
mailerlite-rs = "2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Rustls provides HTTPS by default. Optional features are listed [below](#features).
Version 2 is a breaking redesign; existing users should follow [UPGRADE.md](UPGRADE.md).

## Quick start

Provide your API token through `MAILERLITE_API_TOKEN`, then create or update a
subscriber:

```rust no_run
use mailerlite_rs::{MailerLite, resources::subscribers::UpsertSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite
        .subscribers()
        .upsert(UpsertSubscriber::new("jane@example.com").field("name", "Jane"))
        .await?;

    println!("Subscriber ID: {}", response.data.id);

    Ok(())
}
```

`upsert` creates a subscriber or updates the subscriber with that email address.
Required values go in request constructors; optional values use builder methods.
The SDK accepts the token directly and does not load `.env` files automatically.
An application using `MAILERLITE_API_KEY` can pass that variable's value instead.

Create one client and reuse it. Cloning `MailerLite` shares its HTTP connection
pool, rate-limit pacing, and server cooldowns.

## Resources

Start with a resource accessor, then choose the operation:

| Resource | Accessor |
| --- | --- |
| Subscribers | `mailerlite.subscribers()` |
| Groups | `mailerlite.groups()` |
| Segments | `mailerlite.segments()` |
| Fields | `mailerlite.fields()` |
| Campaigns | `mailerlite.campaigns()` |
| Automations | `mailerlite.automations()` |
| Automation steps | `mailerlite.automation_steps()` |
| Automation triggers | `mailerlite.automation_triggers()` |
| Automation templates | `mailerlite.automation_templates()` |
| Forms | `mailerlite.forms()` |
| Webhook subscriptions | `mailerlite.webhooks()` |
| Batch requests | `mailerlite.batch()` |
| Subscriber imports | `mailerlite.imports()` |
| Reference data | `mailerlite.reference()` |
| Ecommerce | `mailerlite.ecommerce()` |

Subscriber imports include group import creation and progress tracking. Reference
data includes timezones and campaign languages. Ecommerce manages shops and
settings; select a shop with `let shop = mailerlite.ecommerce().shop(shop_id)`
to access its resources:

| Shop resource | Accessor |
| --- | --- |
| Products | `shop.products()` |
| Categories | `shop.categories()` |
| Customers | `shop.customers()` |
| Orders | `shop.orders()` |
| Carts | `shop.carts()` |

Request parameters live under `mailerlite_rs::resources`; response models live
under `mailerlite_rs::models`. A successful operation returns `Response<T>`:
`.data` contains the endpoint's result and `.metadata` contains HTTP status and
selected headers. Documented empty responses use `Response<()>`.

Models preserve exact identifiers and monetary values. Nullable updates
separate omission from explicit null where the endpoint supports it. Known
schemas are typed; heterogeneous batch bodies and unknown future variants stay
explicitly inspectable. Malformed known fields return decoding errors.

Use `mailerlite_rs::Decimal` for exact monetary values and
`mailerlite_rs::StatusCode` for HTTP status comparisons. These re-export the
underlying library types, so basic usage needs no additional direct dependency.
See the [public dependency policy](CONTRIBUTING.md#public-exports-and-dependency-types)
when using HTTP, JSON, serialization, or stream types directly.

## Listing and pagination

List builders end in `.send().await?`. Subscriber lists use cursors, while groups
and many other lists use page numbers:

```rust
use mailerlite_rs::{MailerLite, models::subscribers::SubscriberStatus};

async fn list_subscribers(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    let subscribers = mailerlite.subscribers();
    let first = subscribers
        .list()
        .status(SubscriberStatus::Active)
        .limit(100)
        .send()
        .await?;

    for subscriber in &first.data.data {
        println!("{}", subscriber.id);
    }

    if let Some(cursor) = first.data.next_cursor() {
        let next = subscribers
            .list()
            .status(SubscriberStatus::Active)
            .limit(100)
            .cursor(cursor)
            .send()
            .await?;

        println!("Next page: {} subscribers", next.data.data.len());
    }

    let groups = mailerlite.groups().list().page(1).limit(25).send().await?;

    println!("Groups on this page: {}", groups.data.data.len());

    Ok(())
}
```

Lists return a page inside `Response`, so the items are at `response.data.data`.
Retain filters when requesting another page and use the endpoint's continuation
metadata to decide whether more pages exist.

Enable `stream` for lazy `.into_stream()` pagination. Streams fetch pages as
needed, preserve filters, and detect repeated continuations without collecting
the whole result. See [stream_subscribers.rs](examples/stream_subscribers.rs),
which uses `futures-util` to consume the stream.

Subscriber activity supports manual requests only: backend continuation can
return incomplete or overlapping pages. Webhook management also has no automatic
traversal contract. The SDK does not infer pagination from a full page alone or
follow arbitrary server-provided URLs with your credentials.

## Configuration

`MailerLite::new(token)?` uses these defaults:

| Setting | Default |
| --- | --- |
| API version | `ApiVersion::Latest` |
| Automatic retries | `RetryPolicy::none()` |
| Request timeout | 30 seconds |
| Operation deadline | 60 seconds |
| Global pacing | 1 request / 500 ms |
| Subscriber-import pacing | 1 request / 12 seconds |

The latest version policy omits `X-Version`, and automatic retries are disabled.
The operation deadline covers request serialization, pacing, HTTP attempts,
retry waits, and response decoding. Synchronous JSON work cannot be interrupted;
an overrun returns `Error::Timeout` once that work finishes. Serialization that
exhausts the budget prevents dispatch. Parameter construction and local
validation before dispatch are outside the deadline. Each page or progress
request gets its own budget; a timeout after dispatch does not roll back a write.
Import pacing applies to qualifying subscriber-import operations.

Use the builder to customize configuration or inject a reusable HTTP client:

```rust
use mailerlite_rs::MailerLite;
use std::time::Duration;

fn configured_client(token: String) -> mailerlite_rs::Result<MailerLite> {
    let http = MailerLite::http_client_builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;

    MailerLite::builder(token)
        .http_client(http)
        .request_timeout(Duration::from_secs(30))
        .operation_timeout(Duration::from_secs(60))
        .build()
}
```

Build injected clients with
`MailerLite::http_client_builder()` to retain disabled redirects and underlying
HTTP retries. The SDK cannot inspect or override those policies on an arbitrary
reqwest client. The current SDK uses reqwest 0.13.

To pin API behavior, import `mailerlite_rs::ApiVersion` and add
`.api_version(ApiVersion::pinned("2026-10-30")?)` to the builder.
`DEFAULT_API_VERSION` is available as a release reference date for explicit
pinning; it is not applied automatically.

### Retries and rate limits

To opt into retries, import `mailerlite_rs::RetryPolicy` and add
`.retry_policy(RetryPolicy::safe_reads().max_retries(2))`. Reviewed reads can then
retry with bounded exponential backoff and jitter. A valid `Retry-After` is a
minimum delay; the SDK will not retry early to fit an operation deadline.

Writes and batches are never automatically replayed by the SDK, even with
retries enabled. Shop listing and counting also disable retries because the
server may initialize an internal shop. After a write times out, check its
outcome before submitting it again.

Clones share pacing and server cooldowns. Independently created clients and
separate processes do not coordinate account-wide limits.

## Handling errors

Operations return `mailerlite_rs::Result<T>`. Match `Error::Api` for HTTP failures
and use the structured status, validation details, and response metadata:

```rust
use mailerlite_rs::{Error, MailerLite};

async fn find_subscriber(mailerlite: &MailerLite, id: &str) -> mailerlite_rs::Result<()> {
    match mailerlite.subscribers().get(id).await {
        Ok(response) => {
            println!("{}", response.data.id);

            Ok(())
        }

        Err(Error::Api(error)) => {
            eprintln!("HTTP {}", error.status());

            if error.is_validation() {
                eprintln!("Invalid fields: {}", error.validation_errors().len());
            } else if error.is_rate_limited() {
                eprintln!("Retry delay: {:?}", error.metadata().retry_after());
            }

            Err(Error::Api(error))
        }

        Err(error) => Err(error),
    }
}
```

`ApiError` provides `is_validation()` for 422, `is_unauthorized()` for 401,
`is_rate_limited()` for 429, and `is_server_error()` for 5xx. These classify the
status independently of the response body; they do not enable retries.

`Error::Decode` preserves HTTP metadata and bounded diagnostics for malformed or
unexpected responses. Other variants cover configuration, serialization,
transport, pagination, and operation deadlines. Keep a fallback match arm because
`Error` is non-exhaustive.

API error messages, validation messages, and raw response bodies are available
through explicit accessors and may contain submitted personal information.
API error `Debug` and `Display` omit those values. Client credentials and webhook
secrets are also redacted from debug output.

## Imports and batches

A group import returns a receipt. Use `mailerlite.imports().progress(&receipt)`
to inspect its status or `.wait(...)` to poll with an overall timeout. Dropping
the polling future stops further requests; it does not cancel work the server
has already accepted. See [group_import.rs](examples/group_import.rs).

A successful batch HTTP response can contain failed operations. Check each
item's `code` or `is_success()`, then use `.decode::<T>()` with that operation's
response schema. Results retain submission order. See [batch.rs](examples/batch.rs).

Ecommerce order imports accept `Vec<CreateOrder>` and automatically send
`{"orders": [...]}` to match the backend contract. This intentionally differs
from the bare-array example in the public reference.

## Receiving webhooks

Manage subscriptions through `mailerlite.webhooks()`. Incoming delivery types
live under `mailerlite_rs::webhooks` and require no optional feature. For signature
verification and JSON decoding in your application, use:

```toml
[dependencies]
mailerlite-rs = { version = "2", features = ["webhook-signatures"] }
serde_json = "1"
```

Your HTTP handler supplies the webhook's **signing secret**, the original body
bytes, and the `Signature` header. Use the signing secret rather than the API
token, and verify before parsing JSON. Parsing and reserializing a body changes
the bytes being authenticated.

The following feature-dependent snippet is also compiled as
[verify_webhook.rs](examples/verify_webhook.rs):

```rust ignore
use mailerlite_rs::webhooks::{WebhookPayload, verify_signature};

fn receive(
    secret: &str,
    signature: &str,
    body: &[u8],
) -> Result<WebhookPayload, Box<dyn std::error::Error>> {
    verify_signature(secret, body, signature)?;

    Ok(serde_json::from_slice(body)?)
}
```

Reject missing or invalid signatures before processing events. Verification uses
HMAC-SHA256 with constant-time tag comparison. The header must contain exactly
64 hexadecimal characters without prefixes or whitespace. Empty secrets, invalid
encodings, and mismatches return `SignatureError` variants.

Match `WebhookPayload::Single(event)` or iterate `batch.events` for
`WebhookPayload::Batch(batch)`. The SDK models all 14 publicly documented events;
unknown future events remain inspectable, and malformed known payloads fail
decoding. Keep raw payloads and secrets out of logs.

Signature verification does not establish freshness or prevent duplicate
processing. Make handlers idempotent, queue lengthy work, and return promptly
after accepting a delivery. Your application supplies the HTTP route and event
processing logic.

## Features

| Feature | Purpose |
| --- | --- |
| `rustls-tls` | Default HTTPS backend |
| `native-tls` | Platform TLS backend |
| `stream` | Lazy pagination |
| `tracing` | Transport instrumentation |
| `webhook-signatures` | Incoming webhook signature verification |

Tracing events exclude credentials and request/response bodies.

To select native TLS alone:

```toml
mailerlite-rs = { version = "2", default-features = false, features = ["native-tls"] }
```

Disabling default features without enabling a TLS backend does not independently
provide HTTPS. The SDK uses Rust edition 2024 and supports Rust 1.88 and newer.
Classic API, blocking clients, and browser/WASM support are outside the current
scope. Product assets support JSON URL inputs; multipart file uploads are not
implemented.

Additional automation operations use backend-verified schemas. Their availability
can depend on the deployment; source verification does not guarantee that every
account exposes those routes. They use the same typed APIs and shared transport
as the rest of the SDK.

## Examples and contributing

Browse the [example catalog](examples/README.md) for subscriber management,
campaigns, automation configuration, ecommerce, imports, and more. Existing v1
operation filenames are retained with v2 implementations.

From a repository checkout, compile examples without executing API calls:

```sh
cargo check --examples --all-features
```

To run a selected example, provide its documented environment variables:

```sh
cargo run --example get_subscribers
cargo run --example stream_subscribers --features stream
```

The webhook example verifies and decodes a signed sample event, then rejects a
modified body. It runs offline with a public sample signing secret and needs no
API token:

```sh
cargo run --example verify_webhook --features webhook-signatures
```

Most other examples contact your account when executed. Write operations modify
account data; campaign scheduling can send email. Normal SDK tests use local
HTTP mocks and require no MailerLite token.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, tests, Conventional
Commits, and pull requests. For migration and release history, see
[UPGRADE.md](UPGRADE.md) and [CHANGELOG.md](CHANGELOG.md).

## Support and Feedback

If you find a bug, [submit an issue on GitHub](https://github.com/mailerlite/mailerlite-rust/issues).

You are welcome to create a MailerLite SDK for another programming language.

If you have trouble using our API or SDK, feel free to contact our support team
at [info@mailerlite.com](mailto:info@mailerlite.com).

## License

[The MIT License (MIT)](./LICENSE).
