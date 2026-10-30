# Migrating from 1.x to 2.0

2.0 is a rewrite. The endpoints are the same, but every call is now typed, returns a `Result`,
and sends JSON instead of form data.

## At a glance

| 1.x | 2.0 |
|---|---|
| `MailerLite::new(api_key)` | `MailerLite::new(api_key)?`, which returns `Result` (an invalid key no longer panics) |
| `Data::new().add("email", "a@b.com")` (only `&'static str` values) | Typed bodies: `CreateSubscriber::new("a@b.com").field("name", name).groups([id])` |
| `Parameter::new().add("filter[status]", "active")` | Typed queries: `ListSubscribers::new().status(SubscriberStatus::Active)` |
| `Response { status_code, content: serde_json::Value }` | The model itself (`Subscriber`, `Page<Group>`, ...) or `Error` |
| Panics on network or JSON errors | `Err(Error::Http(..))` / `Err(Error::Decode { .. })` |
| Error bodies replaced by the status' reason | Typed variants: `Error::Validation` (422, ordered field errors), `Error::RateLimited` (429, `retry_after()`), `Error::Unauthorized`, `Error::NotFound`, `Error::Api` (others) |
| Body sent as `application/x-www-form-urlencoded` | Body sent as JSON, so nested `fields`, `groups`, campaign `emails`, orders and batches work |
| `tokio` with every feature pulled in by the SDK | No runtime dependency; bring your own `tokio` |
| reqwest 0.11 + native TLS | reqwest 0.13 + rustls (switch with the `native-tls` feature) |
| Edition 2021 | Edition 2024, MSRV 1.88 |

## Handles

Handle methods are plural now, and e-commerce lives under one scoped handle:

| 1.x | 2.0 |
|---|---|
| `mailerlite.subscriber()` | `client.subscribers()` |
| `mailerlite.campaign()` | `client.campaigns()` |
| `mailerlite.group()` | `client.groups()` |
| `mailerlite.segment()` / `field()` / `form()` | `client.segments()` / `fields()` / `forms()` |
| `mailerlite.automation_step()` / `automation_trigger()` / `automation_template()` | `client.automation_steps()` / `automation_triggers()` / `automation_templates()` |
| `mailerlite.automation()` / `webhook()` | `client.automations()` / `webhooks()` |
| `mailerlite.batch()` / `timezone()` | `client.batch()` / `timezones()` |
| `mailerlite.ecommerce_shop()` | `client.ecommerce().shops()` |
| `mailerlite.ecommerce_setting()` | `client.ecommerce().settings()` |
| `mailerlite.ecommerce_product().get(shop_id, ..)` | `client.ecommerce().shop(shop_id).products().list(..)` |
| `ecommerce_product_variant()` / `ecommerce_category()` / `ecommerce_customer()` | `.shop(id).variants(product_id)` / `.categories()` / `.customers()` |
| `ecommerce_order()` / `ecommerce_cart()` / `ecommerce_cart_item()` | `.shop(id).orders()` / `.carts()` / `.cart_items(cart_id)` |
| `ecommerce_import()` | `.shop(id).imports()` |

## Methods

| 1.x | 2.0 |
|---|---|
| `.get(Parameter)` (list) | `.list(&ListX)`, plus `.list_all(ListX)` to stream every page |
| `.find(id)` | `.get(id)` |
| `.create(Data)` / `.update(id, Data)` | `.create(&CreateX)` / `.update(id, &UpdateX)`; single-field bodies take `&str` (`groups().create("VIP")`) |
| `.delete(id)` | `.delete(id)`, which returns `()` (or the deleted e-commerce resource) |
| `subscriber().create(..)` returning 200/201 in `status_code` | `Upserted { data, created }` |
| `group().get_subscribers(id, ..)` | `groups().subscribers(id, &ListSubscribers)` (same for segments and forms) |
| `campaign().subscribers_activity(id, ..)` | `campaigns().subscriber_activity(id, &ListCampaignActivity)` |
| `automation().subscribers_activity(id, ..)` | `automations().activity(id, &ListAutomationActivity::new(status))` |
| `campaign().languages()` | `campaigns().languages()` |
| `form().get(form_type, ..)` | `forms().list(&FormType::Popup, &ListForms)` |
| `ecommerce_product().change_status(shop, id, Data)` | `.shop(shop).products().enable(id)` / `.disable(id)` |
| `ecommerce_product().assign_categories(..)` | `.shop(shop).products().assign_categories(id, &AssignCategories)` |
| `ecommerce_product_variant().sync(..)` | `.shop(shop).variants(product).create(&[CreateVariant])` (still a sync, see its docs) |
| `automation().ecommerce_stats(id, Parameter)` / `ecommerce_orders(id, Parameter)` | `automations().ecommerce_stats(id, &AutomationEcommerceStatsQuery)` / `ecommerce_orders(id, &ListAutomationOrders::new(TimeFrame::Month))` |

## Example

```rust
// 1.x
let data = Data::new().add("email", "ann@example.com").add("fields[name]", "Ann");
let response = mailerlite.subscriber().create(data).await;
if response.status_code == 201 {
    println!("{}", response.content["data"]["id"]);
}

// 2.0
let upserted = client
    .subscribers()
    .create(&CreateSubscriber::new("ann@example.com").field("name", "Ann"))
    .await?;
if upserted.created {
    println!("{}", upserted.data.id);
}
```

## New in 2.0

- Pagination types (`Page`, `CursorPage`) and lazy `list_all` streams.
- `count()` on subscribers, shops, products, categories, customers, orders, carts and cart items.
- `X-Version` pinning, a configurable timeout and user agent, and bring-your-own `reqwest::Client`.
- E-commerce `with_resource_id()` mode for addressing resources by your own ids.
- `MailerLite::request` for endpoints the SDK doesn't model yet.
- `webhook_signature::verify` (feature `webhook-signature`).

## Automation workflow additions

The automation endpoints added to 1.x also use typed JSON bodies in 2.0:

- `automation().update(id, Data)` becomes `automations().update(id, &UpdateAutomation::new(name))`.
- Step and trigger `get` / `find` become `list` / `get` on the plural handles.
- Step and trigger `create` / `update` take `CreateAutomationStep`, `UpdateAutomationStep`,
  `CreateAutomationTrigger` or `UpdateAutomationTrigger`. Their `data` fields are JSON objects;
  updates replace the complete kind-specific configuration.
- Subscriber actions use `AddAutomationSubscribers`, `AutomationRunAction`,
  `CopyAutomationSubscribers` and `ReaddAutomationSubscribers`. Filters are nested JSON objects,
  with `AutomationRunFilter::new(status)` selecting the run state.
- `test(id, Data)` takes an email address; `update_email_content(id, email_id, Data)` takes HTML.
- `AutomationEmailStatsQuery` supplies dates for email reports. Reports outside the public docs
  keep their complete JSON response as `serde_json::Value`.
- `automation_template().select(id)` becomes `automation_templates().select(id)` and returns an
  `Automation`. Building happens in the background, so wait for
  `trigger_data.building_in_progress` to clear before enabling it.
