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

The errors you handle in code have their own variant, so you can `match` on them:

```rust
use mailerlite_rs::Error;

match client.subscribers().create(&body).await {
    Ok(created) => println!("subscribed {}", created.data.email),

    // 422: field errors, in the order MailerLite sent them
    Err(Error::Validation(invalid)) => {
        for error in invalid.iter() {
            form.set_error(error.field(), error.message()); // "email", "The email must be…"
        }
        if let Some(messages) = invalid.get("email") { /* … */ }
        // "groups.1" -> the second group id you sent
        for error in invalid.iter() {
            if let Some(i) = error.index_after("groups") { /* body.groups[i] is invalid */ }
        }
    }

    // 429: MailerLite allows 120 requests/minute (20/minute for imports)
    Err(Error::RateLimited(limit)) => {
        tokio::time::sleep(limit.retry_after().unwrap_or(Duration::from_secs(60))).await;
    }

    Err(Error::NotFound(_)) => println!("not found"),
    Err(other) => return Err(other), // 401, other 4xx/5xx, network, decode
}
```

Every error also prints as one readable line, listing each field error once:

```text
MailerLite API error 422 Unprocessable Entity: email: The email field must be a valid email address.; groups.1: The selected groups.1 is invalid.
MailerLite API error 429 Too Many Requests: You're being rate limited. Consider using batch requests: … (retry after 42s)
failed to decode MailerLite response at `data.status`: invalid type: null, expected a string
```

The SDK never retries on its own.

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

| Area | Handle | Examples |
|---|---|---|
| Subscribers | `client.subscribers()` | [`get_subscribers`](examples/get_subscribers.rs), [`find_subscriber`](examples/find_subscriber.rs), [`create_subscriber`](examples/create_subscriber.rs), … |
| Groups | `client.groups()` | [`get_groups`](examples/get_groups.rs), [`assign_subscriber_to_group`](examples/assign_subscriber_to_group.rs), [`import_subscribers_to_group`](examples/import_subscribers_to_group.rs), … |
| Segments | `client.segments()` | [`get_segments`](examples/get_segments.rs), [`get_segment_subscribers`](examples/get_segment_subscribers.rs), … |
| Fields | `client.fields()` | [`get_fields`](examples/get_fields.rs), [`create_field`](examples/create_field.rs), … |
| Forms | `client.forms()` | [`get_forms`](examples/get_forms.rs), [`get_form_subscribers`](examples/get_form_subscribers.rs), … |
| Automations | `client.automations()` | [`get_automations`](examples/get_automations.rs), [`get_subscribers_activity_for_automation`](examples/get_subscribers_activity_for_automation.rs), … |
| Campaigns & languages | `client.campaigns()` | [`create_campaign`](examples/create_campaign.rs), [`schedule_campaign`](examples/schedule_campaign.rs), [`get_campaign_languages`](examples/get_campaign_languages.rs), … |
| Webhooks | `client.webhooks()` | [`create_webhook`](examples/create_webhook.rs), [`verify_webhook_signature`](examples/verify_webhook_signature.rs), … |
| Batch | `client.batch()` | [`create_batch`](examples/create_batch.rs) |
| Timezones | `client.timezones()` | [`get_timezones`](examples/get_timezones.rs) |
| E-commerce (shops, products, variants, categories, customers, orders, carts, imports, settings) | `client.ecommerce()` | [`create_ecommerce_shop`](examples/create_ecommerce_shop.rs), [`create_ecommerce_order`](examples/create_ecommerce_order.rs), [`import_ecommerce_products`](examples/import_ecommerce_products.rs), … |

There is one small example per endpoint in [`examples/`](examples), named after the call
(`get_*` lists, `find_*` fetches one, then `create_*`, `update_*`, `delete_*` and so on), plus
[`stream_subscribers`](examples/stream_subscribers.rs), [`handle_errors`](examples/handle_errors.rs)
and [`send_raw_request`](examples/send_raw_request.rs). Replace the `"Your … ID"` placeholders and
run one against your account:

```sh
MAILERLITE_API_KEY=... cargo run --example get_subscribers
```

Every method, request builder and model is documented on [docs.rs](https://docs.rs/mailerlite-rs).

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
