<a href="https://www.mailerlite.com"><img src="https://app.mailerlite.com/assets/images/logo-color.png" width="200px"/></a>

# MailerLite Rust SDK

![Build](https://img.shields.io/github/actions/workflow/status/mailerlite/mailerlite-rust/main.yaml)
![Downloads](https://img.shields.io/crates/d/mailerlite-rs?label=downloads)
![Crates.io](https://img.shields.io/crates/v/mailerlite-rs)
![docs.rs](https://img.shields.io/docsrs/mailerlite-rs)
![License](https://img.shields.io/crates/l/mailerlite-rs)

Typed, async Rust client for the [MailerLite API](https://developers.mailerlite.com/).

- **Typed end to end**: request builders and response models for every documented endpoint,
  with forward-compatible enums (unknown values decode to `Other(String)` instead of failing).
- **No panics**: every call returns `Result`; API errors carry the status, message, per-field
  validation errors and `Retry-After`.
- **Pagination built in**: one page at a time or every item as a lazy `Stream`.
- **Lean**: one pooled `reqwest` client, borrowed resource handles, rustls by default.

Upgrading from 1.x? See [MIGRATION.md](MIGRATION.md).

## Installation

```sh
cargo add mailerlite-rs
cargo add tokio --features macros,rt-multi-thread
```

Minimum supported Rust version: **1.88** (edition 2024).

## Quick start

Create an API key under **Integrations → MailerLite API** in your dashboard.

```rust
use mailerlite_rs::{MailerLite, subscribers::CreateSubscriber};

#[tokio::main]
async fn main() -> mailerlite_rs::Result<()> {
    let client = MailerLite::new(std::env::var("MAILERLITE_API_KEY").unwrap_or_default())?;

    let upserted = client
        .subscribers()
        .create(
            &CreateSubscriber::new("ann@example.com")
                .field("name", "Ann")
                .groups(["4243829086487936"]),
        )
        .await?;

    println!("{} (new: {})", upserted.data.id, upserted.created);
    Ok(())
}
```

## Configuring the client

```rust
use std::time::Duration;
use mailerlite_rs::MailerLite;

let client = MailerLite::builder("api-key")
    .timeout(Duration::from_secs(10))      // default: 30 s per request
    .api_version("2038-01-19")             // pin the API version (sends X-Version)
    .user_agent_suffix("my-app/1.4")       // appended to mailerlite-rs/x.y.z
    .build()?;
```

`MailerLite` is cheap to clone and safe to share across tasks; create one and reuse it. You can
also pass your own `reqwest::Client` with `.http_client(..)`.

## Errors and rate limits

```rust
match client.subscribers().get("ann@example.com").await {
    Ok(subscriber) => println!("{}", subscriber.status),
    Err(err) if err.is_not_found() => println!("no such subscriber"),
    Err(err) if err.is_rate_limited() => {
        let wait = err.as_api().and_then(|api| api.retry_after);
        println!("slow down, retry after {wait:?}");
    }
    Err(err) if err.is_validation() => {
        for (field, messages) in &err.as_api().expect("API error").errors {
            println!("{field}: {}", messages.join(", "));
        }
    }
    Err(err) => return Err(err),
}
```

MailerLite allows 120 requests per minute (5 per minute for imports). The SDK never retries on its
own, so back off using `retry_after`.

## Pagination

```rust
use futures_util::TryStreamExt;
use mailerlite_rs::{Sort, groups::{GroupSort, ListGroups}};

// One page.
let page = client
    .groups()
    .list(&ListGroups::new().sort(Sort::desc(GroupSort::ActiveCount)).limit(25))
    .await?;
println!("{} groups in total", page.meta.total);

// Every item, fetched page by page as you consume the stream (`stream` feature).
let groups = client.groups();
let mut all = std::pin::pin!(groups.list_all(ListGroups::new()));
while let Some(group) = all.try_next().await? {
    println!("{}", group.name);
}
```

## Resources

| Area | Handle | Example |
|---|---|---|
| Subscribers | `client.subscribers()` | [`examples/subscribers.rs`](examples/subscribers.rs) |
| Groups | `client.groups()` | [`examples/groups.rs`](examples/groups.rs) |
| Segments | `client.segments()` | [`examples/segments.rs`](examples/segments.rs) |
| Fields | `client.fields()` | [`examples/fields.rs`](examples/fields.rs) |
| Forms | `client.forms()` | [`examples/forms.rs`](examples/forms.rs) |
| Automations | `client.automations()` | [`examples/automations.rs`](examples/automations.rs) |
| Campaigns & languages | `client.campaigns()` | [`examples/campaigns.rs`](examples/campaigns.rs) |
| Webhooks | `client.webhooks()` | [`examples/webhooks.rs`](examples/webhooks.rs) |
| Batch | `client.batch()` | [`examples/batch.rs`](examples/batch.rs) |
| Timezones | `client.timezones()` | [`examples/timezones.rs`](examples/timezones.rs) |
| E-commerce (shops, products, variants, categories, customers, orders, carts, imports, settings) | `client.ecommerce()` | [`examples/ecommerce.rs`](examples/ecommerce.rs), [`examples/ecommerce_resource_ids.rs`](examples/ecommerce_resource_ids.rs) |

Every method, request builder and model is documented on [docs.rs](https://docs.rs/mailerlite-rs).
The examples run against your real account (they clean up after themselves):

```sh
MAILERLITE_API_KEY=... cargo run --example subscribers
```

Endpoints the SDK doesn't model yet are reachable with the same auth and error handling:

```rust
use mailerlite_rs::Method;

let body: serde_json::Value = client.request(Method::GET, "subscribers").query(&[("limit", "5")]).send().await?;
```

## Features

| Feature | Default | Description |
|---|---|---|
| `rustls` | yes | TLS via rustls |
| `native-tls` | | TLS via the platform's native stack |
| `stream` | yes | `list_all` helpers returning a `futures` `Stream` |
| `webhook-signature` | | `webhook_signature::verify` to check the `Signature` header of incoming webhooks |

```toml
mailerlite-rs = { version = "2", default-features = false, features = ["native-tls"] }
```

## Contributing

See [CLAUDE.md](CLAUDE.md) for the layout, conventions and checks, and
[`.claude/skills/add-endpoint`](.claude/skills/add-endpoint/SKILL.md) for the step-by-step recipe for
adding an endpoint. Commits follow [Conventional Commits](https://www.conventionalcommits.org/);
releases are automated with release-please.

## License

[MIT](LICENSE)
