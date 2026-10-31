# Upgrading from 1.x to 2.0

Version 2 is a breaking redesign of the 1.1.0 API. The crate name remains
`mailerlite-rs` and the Rust import name remains `mailerlite_rs`. This guide covers
application migration; the [example catalog](examples/README.md) provides the
updated version of every retained v1 example.

## Dependencies and Rust version

Use Rust **1.88 or newer**, including in CI. The SDK uses edition 2024; your
application does not need to change its own edition to depend on it.

```toml
[dependencies]
mailerlite-rs = "2"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

This is the dependency declaration for the 2.0 release. When testing an
unpublished checkout, use a local path dependency instead. Run
`cargo update -p mailerlite-rs` after updating the version requirement.

| Feature | Purpose |
| --- | --- |
| `rustls-tls` (default) | HTTPS through Rustls; v1 used reqwest's default native TLS |
| `native-tls` | HTTPS through the platform TLS backend |
| `stream` | Lazy pagination with `.into_stream()` |
| `tracing` | Optional tracing instrumentation |
| `webhook-signatures` | Incoming webhook HMAC verification; payload models need no feature |

To use native TLS, replace the dependency declaration with:

```toml
mailerlite-rs = { version = "2", default-features = false, features = ["native-tls"] }
```

Disabling default features without enabling a TLS backend does not provide HTTPS
by itself. There is no blocking feature; API calls remain asynchronous and use
Tokio. If your application names reqwest types used by the SDK, update that
dependency from reqwest 0.11 to **0.13**. A reqwest 0.11 client cannot be injected
into this SDK.

`mailerlite_rs::Decimal` and `mailerlite_rs::StatusCode` now provide convenient
imports for the exact decimal and HTTP status types already used by the SDK.
They are re-exports, so existing compatible upstream imports continue to work.
No wrapper types, conversions, dependency upgrades, or new features are needed.
For example, use `Decimal::new(1999, 2)` for a price of `19.99` and
`StatusCode::UNPROCESSABLE_ENTITY` when comparing an API error's status.

Public re-exports now list their names explicitly. All existing identifier,
automation-configuration, and subscriber-activity import paths are retained.
When naming other exposed dependency types, use the compatible versions in
the [public dependency policy](CONTRIBUTING.md#public-exports-and-dependency-types).

## Client construction and configuration

`MailerLite::new` now returns `Result<MailerLite, Error>`. Read environment
variables in your application; the SDK does not load `.env` automatically.

```rust
use mailerlite_rs::{MailerLite, resources::subscribers::UpsertSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mailerlite = MailerLite::new(std::env::var("MAILERLITE_API_TOKEN")?)?;

    let response = mailerlite
        .subscribers()
        .upsert(UpsertSubscriber::new("jane@example.com").field("name", "Jane"))
        .await?;

    println!("{}", response.data.id);

    Ok(())
}
```

An existing environment variable named `MAILERLITE_API_KEY` works too: pass its
value to the same constructor. Keep one client per configuration; clones share
connections, pacing, and server cooldowns.

Use the builder for a custom endpoint, timeouts, or an injected HTTP client:

```rust
use mailerlite_rs::{ApiVersion, DEFAULT_API_VERSION, MailerLite};
use std::time::Duration;

fn configured_client(token: String) -> mailerlite_rs::Result<MailerLite> {
    let http = MailerLite::http_client_builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;

    MailerLite::builder(token)
        .http_client(http)
        .base_url("http://localhost:8080/api")
        // Optional: pin a date instead of using the latest version.
        .api_version(ApiVersion::pinned(DEFAULT_API_VERSION)?)
        .request_timeout(Duration::from_secs(30))
        .operation_timeout(Duration::from_secs(60))
        .build()
}
```

The base URL must include the API prefix. Omit `.base_url(...)` to use
`https://connect.mailerlite.com/api`. Prefer `MailerLite::http_client_builder()`
when injecting a client: it disables redirects and underlying automatic retries.
The SDK cannot inspect or override those policies on an arbitrary injected client.

The operation timeout covers request serialization, pacing, HTTP attempts, retry
waits, and response decoding. Synchronous JSON work is checked after it finishes;
it cannot be interrupted mid-call. If serialization exhausts the budget, no
request is sent. Parameter construction and local validation are outside this
budget, and each page or progress request starts a new one. A timeout after
dispatch does not imply that a write was rolled back.

Like v1, v2 omits `X-Version` by default. `ApiVersion::Latest` selects the server's
latest API behavior. To opt into a dated version, use `ApiVersion::pinned(...)`.
`DEFAULT_API_VERSION` is a release reference date for explicit pinning, not the
default policy; the configuration example above deliberately opts into it.

Automatic retries remain **disabled by default**. Opt in with
`.retry_policy(RetryPolicy::safe_reads().max_retries(2))`, importing
`mailerlite_rs::RetryPolicy`. Only reviewed reads can retry. Writes and batches
are never automatically replayed by the SDK. A write timeout can leave the
server-side outcome uncertain; check state before manually repeating it.

## Resource and method names

Resource handles now borrow the client. Obtain them through `MailerLite` rather
than constructing the old public resource structs. Request types live under
`resources::<resource>`; response models live under `models::<resource>`.
The old `data::Data`, `parameter::Parameter`, and public `client::Client` APIs
are removed.

Every v1 top-level resource accessor maps as follows. `shop` below means
`mailerlite.ecommerce().shop(shop_id)`.

| V1 accessor or operation | V2 replacement |
| --- | --- |
| `.subscriber()` | `.subscribers()` |
| `.campaign()` | `.campaigns()` |
| `.group()` | `.groups()` |
| `.segment()` | `.segments()` |
| `.field()` | `.fields()` |
| `.form()` | `.forms()` |
| `.automation()` | `.automations()` |
| `.automation_trigger()` | `.automation_triggers()` |
| `.automation_step()` | `.automation_steps()` |
| `.automation_template()` | `.automation_templates()` |
| `.webhook()` | `.webhooks()` |
| `.batch()` | `.batch()` with typed batch requests |
| `.timezone().get()` | `.reference().timezones()` |
| `.ecommerce_shop()` | `.ecommerce()` |
| `.ecommerce_product()` | `shop.products()` |
| `.ecommerce_product_variant()` | `shop.products().variants(product_id)` |
| `.ecommerce_category()` | `shop.categories()` |
| `.ecommerce_customer()` | `shop.customers()` |
| `.ecommerce_order()` | `shop.orders()` |
| `.ecommerce_cart()` | `shop.carts()` |
| `.ecommerce_cart_item()` | `shop.carts().items(cart_id)` |
| `.ecommerce_import().products(shop_id, data)` | `shop.products().import(typed_products)` |
| `.ecommerce_import().categories(shop_id, data)` | `shop.categories().import(typed_categories)` |
| `.ecommerce_import().customers(shop_id, data)` | `shop.customers().import(typed_customers)` |
| `.ecommerce_import().orders(shop_id, data)` | `shop.orders().import(typed_orders)` |
| `.ecommerce_setting().find()` | `.ecommerce().settings()` |

Method names also change: collection `.get(parameters).await` generally becomes
`.list().<filters>().send().await?`; single-resource `.find(id).await` becomes
`.get(id).await?`. Subscriber `.create(data)` becomes
`.upsert(UpsertSubscriber::new(email))`. Required arguments belong in typed
constructors; optional values use setters. JSON bodies replace v1's form-encoded
`Data` payloads. Resource-specific methods can have additional changes; use the
[matching example](examples/README.md) for each operation.

For external ecommerce identifiers, use
`mailerlite.ecommerce().shop(shop_id).with_resource_ids()` before selecting the
nested resource. The shop itself still takes its MailerLite shop ID.

## Listing and pagination

Repeated setters replace the previous value for that parameter. Validation uses
the final request state for both `.send()` and `.into_stream()`: for example,
`.limit(0).limit(25)` uses 25. Correcting the limit does not clear an invalid page,
unknown subscriber status, or an option unsupported by the selected endpoint.

Group and field list builders take an explicit `resources::SortDirection`:
`.sort(GroupSort::Name, SortDirection::Ascending)` or
`.sort(FieldSort::Name, SortDirection::Descending)`. Import the shared direction
from `mailerlite_rs::resources::SortDirection` and the field enum from its resource
module. If you used the early v2 boolean argument, replace `false` with
`SortDirection::Ascending` and `true` with `SortDirection::Descending`. This is a
source-breaking change; the query values remain `name` and `-name`, respectively.

Lazy request builders are `#[must_use]`: discarding one now warns, or fails
compilation when `unused_must_use` is denied. Call `.send().await` to execute a
request, or enable `stream` and consume `.into_stream()`. Discarded client
configuration builders also warn; call `.build()` to construct the client.
Use `let _ = ...` only when intentionally discarding a builder; it does not
execute the operation.

V1 listing and manual continuation used string parameters and raw JSON:

```rust
use mailerlite_rs::{MailerLite, parameter::Parameter};

async fn list_active(mailerlite: &MailerLite) {
    let params = Parameter::new()
        .add("filter[status]", "active")
        .add("limit", "100");
    let response = mailerlite.subscriber().get(params).await;

    println!("{}", response.content["data"]);
    // Continuation was read manually from JSON for a subsequent request.
    let next_cursor = response.content["meta"]["next_cursor"].as_str();

    println!("Next cursor: {next_cursor:?}");
}
```

V2 uses typed filters, items, and continuation metadata:

```rust
use mailerlite_rs::{MailerLite, models::subscribers::SubscriberStatus};

async fn list_active(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    let subscribers = mailerlite.subscribers();
    let mut cursor = None;

    loop {
        let mut request = subscribers
            .list()
            .status(SubscriberStatus::Active)
            .limit(100);

        if let Some(value) = cursor {
            request = request.cursor(value);
        }

        let response = request.send().await?;

        for subscriber in &response.data.data {
            println!("{}", subscriber.id);
        }
        cursor = response.data.next_cursor().map(str::to_owned);

        if cursor.is_none() {
            break;
        }
    }

    Ok(())
}
```

`Response<T>.data` holds the endpoint's model. For list endpoints, that model is
usually `CursorPage<T>` or `NumberedPage<T>`, whose own `.data` holds the items;
this is why list iteration uses `response.data.data`. Numbered endpoints use
`.page(n)` instead of `.cursor(...)`. Empty items alone do not establish the end
when the server supplies a valid next continuation.

For automatic traversal, enable `stream` and add `futures-util = "0.3"` to your
application dependencies:

```rust
use futures_util::TryStreamExt;
use mailerlite_rs::MailerLite;

async fn stream_subscribers(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    let subscribers = mailerlite.subscribers();
    let stream = subscribers.list().limit(100).into_stream();
    tokio::pin!(stream);

    while let Some(subscriber) = stream.try_next().await? {
        println!("{}", subscriber.id);
    }

    Ok(())
}
```

Streams fetch pages lazily and validate continuation progress. They do not load
all pages into memory. Subscriber activity remains manual via
`.subscribers().activity(id).page(1).limit(100).send()`: the backend cannot reliably
provide subsequent pages. Webhook management also has no automatic traversal
contract. Consult each resource's rustdoc for endpoint-specific limitations.

## Responses and errors

V1 returned a nongeneric `Response` even for HTTP failures such as 422:

```rust
use mailerlite_rs::MailerLite;

async fn find_subscriber(mailerlite: &MailerLite, id: &str) {
    let response = mailerlite.subscriber().find(id).await;

    if response.status_code.as_u16() == 422 {
        println!("Validation failed");
    } else if response.status_code.is_success() {
        println!("{}", response.content["data"]["id"]);
    }
}
```

V2 returns `Result<Response<T>, Error>`. HTTP failures are `Error::Api`; successful
responses expose status through `response.metadata.status`. Match errors instead
of indexing a successful response's JSON:

```rust
use mailerlite_rs::{Error, MailerLite};

async fn find_subscriber(mailerlite: &MailerLite, id: &str) -> mailerlite_rs::Result<()> {
    match mailerlite.subscribers().get(id).await {
        Ok(response) => println!("{}: {}", response.metadata.status, response.data.id),

        Err(Error::Api(error)) if error.is_validation() => {
            // Handle 422 using the structured field-to-messages map.
            let invalid_fields = error.validation_errors().keys().count();

            eprintln!("Validation failed for {invalid_fields} fields");
            return Err(Error::Api(error));
        }

        Err(Error::Api(error)) if error.is_rate_limited() => {
            eprintln!(
                "Rate limited; retry delay: {:?}",
                error.metadata().retry_after()
            );
            return Err(Error::Api(error));
        }

        Err(error) => return Err(error),
    }

    Ok(())
}
```

Use `ApiError::status()`, `message()`, `validation_errors()`, and `metadata()` for
diagnostics. `is_unauthorized()` and `is_server_error()` provide additional
classification; inspect `status()` when distinguishing 401 from 403.
Message/body accessors may reveal submitted personal
data; `Debug` and `Display` deliberately omit API bodies and messages.

Transport, configuration, serialization, timeout, pagination, and decoding
failures are recoverable error variants. `Error::Decode` retains HTTP metadata
and bounded diagnostics for unexpected responses. `Error` is non-exhaustive,
so keep a fallback match arm. Empty documented successes return `Response<()>`;
some ecommerce deletes return a typed deleted resource instead.

## Omitted fields, nulls, and updates

Omitting an optional setter usually omits the corresponding JSON key; it does
not send null. Use the endpoint's explicit clearing operation when needed:

```rust
use mailerlite_rs::{
    models::subscribers::FieldValue,
    resources::{automation_steps::UpdateStep, subscribers::UpdateSubscriber},
};

fn update_parameters() -> (UpdateSubscriber, UpdateStep) {
    // No groups setter: preserve subscriber group membership.
    // No company field: leave that custom field untouched.
    // Explicit null: clear the documented nullable name field.
    let subscriber = UpdateSubscriber::new().field("name", FieldValue::Null);

    // Omitting parent_id leaves the position unchanged; clear_parent sends null.
    let step = UpdateStep::new().clear_parent();

    (subscriber, step)
}
```

Subscriber PUT updates and POST upserts differ: `UpdateSubscriber::groups`
replaces membership, including clearing it with `.groups(Vec::<String>::new())`;
supplied upsert groups add membership. Omitted groups preserve membership.

Public nullable configuration fields use `Patch<T>`: `Unset` omits the field,
`Null` sends JSON null, and `Value(value)` sends a value. These describe wire
representation, not a universal preservation guarantee. In particular, automation
step configuration updates replace configuration; supply the complete intended
configuration because omitted settings can be cleared or defaulted by the backend.

## Imports and retained operations

Group imports return receipts and must be polled explicitly. Use
`mailerlite.imports().progress(&receipt)` or bounded `.wait(...)`; see
[the group import example](examples/group_import.rs). A timeout stops waiting,
not work already accepted by the server. Batch HTTP success can contain failed
items: inspect each item's `code` or `is_success()` before calling `decode::<T>()`.

Order imports accept `Vec<CreateOrder>` and serialize it as `{"orders": [...]}`
to match the backend. The SDK adds this wrapper automatically. This deliberately
differs from the bare-array example in the public API documentation.

All nonduplicate v1 examples are retained and migrated to v2. Automation steps,
triggers, templates, actions, and reports use schemas verified against the
MailerLite backend. Use `.automation_steps()`, `.automation_triggers()`, and
`.automation_templates()` for the additional resources; action and report methods
are on `.automations()`. These use typed JSON requests and responses through the
same shared transport, with no v1 compatibility helpers. Additional automation
routes depend on the deployment; backend schema verification does not guarantee
their availability on every account. Classic API remains outside scope.

## Typed automation configuration

Replace v1 string-keyed `Data` configuration with concrete models under
`models::automation_configuration`, passed through `.configuration(...)` or
`CreateStep::configured(...)`.
The six step variants are email, delay, condition, A/B test, action, and webhook;
trigger configuration covers the 13 verified configurable trigger types.

```rust
use mailerlite_rs::models::automation_configuration::{
    DelayConfiguration, DelayUnit, StepConfiguration,
};
use mailerlite_rs::resources::automation_steps::{CreateStep, UpdateStep};

fn configuration() -> mailerlite_rs::Result<(CreateStep, UpdateStep)> {
    let create = CreateStep::configured(StepConfiguration::Delay(DelayConfiguration::new(
        DelayUnit::Days,
        1,
    )))?;
    let update = UpdateStep::new().configuration(StepConfiguration::Delay(
        DelayConfiguration::new(DelayUnit::Days, 2),
    ))?;

    Ok((create, update))
}
```

`CreateStep::configured` and `CreateAutomationTrigger::configured` select the
matching type. When using `.configuration(...)` on a creation or replacement
definition with an existing type, a mismatch returns a local configuration error.
Updates to an existing step or trigger have no type in the endpoint path; the
server checks that the supplied configuration matches that resource. The SDK
does not add a hidden GET to infer it.

Configuration fields use `Patch<T>` where omission and explicit null differ.
The backend still validates ownership, plan limits and subtype-specific rules.
Do not assume a null setting means clearing: some server settings apply a
fallback. Generic future fields use fallible `.extension_data(...)` or
`.extension_setting(...)`; these reject known keys so they cannot bypass the
typed setters. Response extensions are explicit and redacted from debug output.

## Typed response models and incoming webhooks

Server-owned resource, report, activity, webhook-delivery, and enriched response
configuration structs are `#[non_exhaustive]`. They can gain fields in future
SDK releases. Read fields directly, or include `..` when destructuring. External
struct literals (including `..Default::default()`) no longer compile for these
types. Existing `Default` implementations still work and public fields remain
writable. Deserialize sanitized JSON when building response fixtures:

```rust
use mailerlite_rs::models::fields::Field;

fn field_fixture() -> serde_json::Result<Field> {
    serde_json::from_str(
        r#"{"id":"123","name":"Company","key":"company","type":"text"}"#,
    )
}

fn field_name(field: Field) -> String {
    let Field { name, .. } = field;

    name
}
```

`SubscriberStatus` and `WebformActivity` now join the existing non-exhaustive
response enums. Add a fallback arm to matches. `WebformActivity::Form` and
`WebformActivity::Site` also require `..` in patterns because their fields may
grow:

```rust
use mailerlite_rs::models::subscribers::{SubscriberStatus, WebformActivity};

fn status_label(status: &SubscriberStatus) -> &str {
    match status {
        SubscriberStatus::Active => "active",
        SubscriberStatus::Unsubscribed => "unsubscribed",
        SubscriberStatus::Unconfirmed => "unconfirmed",
        SubscriberStatus::Bounced => "bounced",
        SubscriberStatus::Junk => "junk",
        SubscriberStatus::Unknown(value) => value,
        _ => "unrecognized SDK status",
    }
}

fn form_id(activity: &WebformActivity) -> Option<&str> {
    match activity {
        WebformActivity::Form { form_id, .. } => Some(form_id),
        _ => None,
    }
}
```

This is a Rust source compatibility policy, not relaxed JSON validation.
`SubscriberStatus::Unknown` still preserves future server strings and rejects
them in requests. Unknown `webform_type` values still fail decoding. No new
wire values, fallback variants, or default fields are introduced by the
attribute.

Request parameters and configuration remain constructible. Finite representations
such as `Patch`, `FieldValue`, configuration IDs/scalars, weekdays, and
array-versus-map enums remain exhaustively matchable. Reusable response/page and
batch envelopes, HTTP/pagination metadata, import receipts, fixed rates/counts,
and shared request `RelatedObject` values also retain their construction APIs.
Treat changes to those closed shapes as compatibility changes rather than
assuming fields or variants can be freely added.

- `Form.double_optin` is `Option<bool>`.
- `EcommerceStats.conversion_rate` is `Option<String>` because shop and subscriber
  reports may omit it. Use `.as_deref()` to inspect a returned rate; absence is
  distinct from a reported zero rate.
- `ImportReport` exposes outcome samples/counts and named metadata rather than raw JSON indexing. Sample IDs identify import-log rows.
- `SubscriberActivity.properties` is `ActivityProperties`; match the known
  variants, or explicitly inspect an unknown future payload. Provider-specific
  delivery-error context retains an extension map.
- Product assets, settings, timezone and order statistics have concrete models
  under `models::ecommerce`. Request settings remain separate under
  `resources::ecommerce::params`. JSON URL assets use `SaveProductAsset::url`;
  uploading file bytes requires a separate multipart contract.
- Step and trigger response properties, shared trigger state, and template
  workflows expose concrete fields. Stored configuration identifiers can
  preserve either the original integer or string representation.
- `NumberedMeta.aggregations` is `Option<BTreeMap<String, u64>>`: resource
  category names remain dynamic, while counts decode as nonnegative integers
  or digit strings without floating-point conversion.

Mixed batch items still carry heterogeneous bodies. Use `is_success()` and
`decode::<T>()` per item; a batch-wide universal response schema would not match
the API. Unknown event variants and future extension properties remain explicit.

Incoming webhook types live under `mailerlite_rs::webhooks`; management stays
under `mailerlite.webhooks()`. Enable `webhook-signatures` to use
`verify_signature(secret, raw_body, signature)`. See [receiving webhooks](README.md#receiving-webhooks).

Request and response model fields follow verified wire types. Resource
identifiers preserve exact values; money uses decimals; timezone-free timestamp
strings are not assigned a timezone. Unknown response fields are generally
ignored or retained in explicit extension fields, while supported discriminator
models retain future values where appropriate. Malformed known values remain
decoding errors.
