# Examples

160 standalone examples for the MailerLite 2.0 SDK. Each file has its own entry point, with required environment variables documented at the top. Async examples use Tokio. All 143 v1 operation filenames are retained; the code uses the v2 API.

## Running an example

Provide `MAILERLITE_API_TOKEN` through your environment, then run an example by filename without `.rs`:

```sh
cargo run --example get_subscribers

export MAILERLITE_SUBSCRIBER_EMAIL='jane@example.com'
cargo run --example create_subscriber

cargo run --example stream_subscribers --features stream

# Offline verification and tampering example; no API token is needed.
cargo run --example verify_webhook --features webhook-signatures
```

Examples read environment variables directly; they do not automatically load a `.env` file. Set the additional variables listed in the selected file, such as `MAILERLITE_GROUP_ID` or `MAILERLITE_SHOP_ID`. Sample names and values can be edited before running.

Update examples require explicit replacement values. In addition to the token and
resource IDs listed in each file header, supply the following environment values:

| Example | Replacement values |
| --- | --- |
| `update_ecommerce_shop` | `MAILERLITE_SHOP_NAME`, `MAILERLITE_SHOP_URL`, `MAILERLITE_SHOP_CURRENCY` |
| `update_ecommerce_product` | `MAILERLITE_PRODUCT_NAME`, `MAILERLITE_PRODUCT_PRICE` |
| `update_ecommerce_category` | `MAILERLITE_CATEGORY_NAME` |
| `update_ecommerce_product_variant` | `MAILERLITE_VARIANT_NAME`, `MAILERLITE_VARIANT_ATTRIBUTES`, `MAILERLITE_VARIANT_PRICE`, `MAILERLITE_VARIANT_PRICE_TYPE` |
| `update_ecommerce_cart_item` | `MAILERLITE_PRODUCT_ID`, `MAILERLITE_CART_ITEM_QUANTITY`, `MAILERLITE_CART_ITEM_PRICE` |
| `update_ecommerce_cart` | `MAILERLITE_CHECKOUT_URL` |
| `update_ecommerce_customer` | `MAILERLITE_CUSTOMER_EMAIL` |
| `update_ecommerce_order` | `MAILERLITE_ORDER_STATUS` |
| `update_campaign` | `MAILERLITE_CAMPAIGN_NAME`, `MAILERLITE_CAMPAIGN_SUBJECT`, `MAILERLITE_SENDER_EMAIL`, `MAILERLITE_SENDER_NAME` |
| `update_subscriber` | `MAILERLITE_SUBSCRIBER_NAME` |
| `update_webhook` | `MAILERLITE_WEBHOOK_NAME` |
| `update_automation` | `MAILERLITE_AUTOMATION_NAME` |
| `update_automation_step` | `MAILERLITE_DELAY_DAYS`, `MAILERLITE_RESCHEDULE_WAITING_SUBSCRIBERS` |
| `update_automation_trigger` | `MAILERLITE_GROUP_ID` |
| `update_automation_email_content` | `MAILERLITE_EMAIL_HTML` |
| `update_field` | `MAILERLITE_FIELD_NAME` |
| `update_form` | `MAILERLITE_FORM_NAME` |
| `update_group` | `MAILERLITE_GROUP_NAME` |
| `update_segment` | `MAILERLITE_SEGMENT_NAME` |

Prices are decimal text; quantities and delay days are unsigned integers.
`MAILERLITE_VARIANT_ATTRIBUTES` is a complete JSON object of string values, such
as `'{"color":"Blue","size":"Large"}'`; an empty object replaces the map with
no attributes. Variant price type must be `fixed` or `increment`, order status
must be `pending` or `complete`, and the reschedule flag must be `true` or `false`.
The automation step example replaces the delay configuration with the supplied
number of days. These values overwrite the corresponding fields on the selected
resource, so choose them deliberately.

Most examples make the operation described in the file against your account. Create, update, delete, import, assignment, and scheduling examples change account data; scheduling can send email. Normal CI only compiles examples and requires no API token. `configuration` builds a client without making an API request. `verify_webhook` verifies and decodes a signed synthetic event, then demonstrates rejection of a modified body. It needs no API token or running server.

List examples demonstrate one page unless their purpose is pagination. See `pagination` for manual paging and `stream_subscribers` for lazy automatic pagination. E-commerce examples use MailerLite identifiers by default; the SDK also supports explicit external resource identifiers through `with_resource_ids()`.

See the [README](../README.md) for configuration, pagination, and webhook
verification. Resource rustdoc describes supported operations and their limitations.

The `configuration` example builds an injected HTTP client with the SDK's
redirect and retry defaults. It requires `MAILERLITE_API_TOKEN` and uses
`ApiVersion::Latest` by default. Optionally set `MAILERLITE_API_VERSION` to your
chosen `YYYY-MM-DD` date for explicit pinning. Only a missing variable selects
`Latest`; present but empty, whitespace-only, or invalid values are errors.
`ApiVersion::pinned` trims surrounding whitespace for both this example and direct
library calls.
Choose that date using MailerLite's [versioning guidance](https://developers.mailerlite.com/getting-started#versioning).

The `group_import` example polls for up to five minutes. A timeout stops waiting,
not accepted server work; retain the receipt to inspect progress later. Likewise,
check a timed-out write's outcome before replaying it.

The `batch` example preserves submission order and decodes successful items using
their endpoint-specific schemas. Check `is_success()` before `decode::<T>()`.
Webhook management requests are unsupported by the batch endpoint; incoming
webhook delivery batches are a separate format. Avoid logging arbitrary item
bodies, which may contain personal data.

## Getting started and SDK behavior

- [batch](batch.rs)
- [configuration](configuration.rs)
- [ecommerce](ecommerce.rs)
- [group_import](group_import.rs)
- [handle_errors](handle_errors.rs)
- [pagination](pagination.rs)
- [quick_start](quick_start.rs)
- [stream_subscribers](stream_subscribers.rs)
- [verify_webhook](verify_webhook.rs) — requires `webhook-signatures`; offline

## Subscribers

- [count_subscribers](count_subscribers.rs)
- [create_subscriber](create_subscriber.rs)
- [delete_subscriber](delete_subscriber.rs)
- [find_subscriber](find_subscriber.rs)
- [forget_subscriber](forget_subscriber.rs)
- [get_subscriber_activity](get_subscriber_activity.rs)
- [get_subscribers](get_subscribers.rs)
- [update_subscriber](update_subscriber.rs)

## Groups

- [assign_subscriber_to_group](assign_subscriber_to_group.rs)
- [create_group](create_group.rs)
- [delete_group](delete_group.rs)
- [get_group_subscribers](get_group_subscribers.rs)
- [get_groups](get_groups.rs)
- [unassign_subscriber_from_group](unassign_subscriber_from_group.rs)
- [update_group](update_group.rs)

## Segments

- [delete_segment](delete_segment.rs)
- [get_segment_subscribers](get_segment_subscribers.rs)
- [get_segments](get_segments.rs)
- [update_segment](update_segment.rs)

## Fields

- [create_field](create_field.rs)
- [delete_field](delete_field.rs)
- [get_fields](get_fields.rs)
- [update_field](update_field.rs)

## Campaigns

- [cancel_campaign](cancel_campaign.rs)
- [create_campaign](create_campaign.rs)
- [delete_campaign](delete_campaign.rs)
- [find_campaign](find_campaign.rs)
- [get_campaigns](get_campaigns.rs)
- [get_subscribers_activity_for_campaign](get_subscribers_activity_for_campaign.rs)
- [schedule_campaign](schedule_campaign.rs)
- [update_campaign](update_campaign.rs)

## Automations

- [add_automation_subscribers_to_group](add_automation_subscribers_to_group.rs)
- [add_subscribers_to_automation](add_subscribers_to_automation.rs)
- [cancel_automation_for_subscribers](cancel_automation_for_subscribers.rs)
- [copy_automation](copy_automation.rs)
- [create_automation](create_automation.rs)
- [delete_automation](delete_automation.rs)
- [disable_automation](disable_automation.rs)
- [enable_automation](enable_automation.rs)
- [find_automation](find_automation.rs)
- [get_automation_ecommerce_orders](get_automation_ecommerce_orders.rs)
- [get_automation_ecommerce_stats](get_automation_ecommerce_stats.rs)
- [get_automation_email_content](get_automation_email_content.rs)
- [get_automation_email_ecommerce_stats](get_automation_email_ecommerce_stats.rs)
- [get_automation_email_report_stats](get_automation_email_report_stats.rs)
- [get_automation_email_stats](get_automation_email_stats.rs)
- [get_automation_email_unsubscribe_reasons](get_automation_email_unsubscribe_reasons.rs)
- [get_automations](get_automations.rs)
- [get_subscribers_activity_for_automation](get_subscribers_activity_for_automation.rs)
- [readd_subscribers_to_automation](readd_subscribers_to_automation.rs)
- [skip_automation_delay](skip_automation_delay.rs)
- [test_automation](test_automation.rs)
- [update_automation](update_automation.rs)
- [update_automation_email_content](update_automation_email_content.rs)

## Automation steps

- [copy_automation_step](copy_automation_step.rs)
- [create_automation_step](create_automation_step.rs)
- [delete_automation_step](delete_automation_step.rs)
- [find_automation_step](find_automation_step.rs)
- [get_automation_ab_step_stats](get_automation_ab_step_stats.rs)
- [get_automation_ab_steps_stats](get_automation_ab_steps_stats.rs)
- [get_automation_step_run_counts](get_automation_step_run_counts.rs)
- [get_automation_step_subscribers](get_automation_step_subscribers.rs)
- [get_automation_steps](get_automation_steps.rs)
- [get_automation_steps_run_counts](get_automation_steps_run_counts.rs)
- [reorder_automation_steps](reorder_automation_steps.rs)
- [select_automation_ab_test_winner](select_automation_ab_test_winner.rs)
- [update_automation_step](update_automation_step.rs)

## Automation triggers

- [create_automation_trigger](create_automation_trigger.rs)
- [delete_automation_trigger](delete_automation_trigger.rs)
- [find_automation_trigger](find_automation_trigger.rs)
- [get_automation_trigger_run_counts](get_automation_trigger_run_counts.rs)
- [get_automation_trigger_subscribers](get_automation_trigger_subscribers.rs)
- [get_automation_triggers](get_automation_triggers.rs)
- [get_automation_triggers_run_counts](get_automation_triggers_run_counts.rs)
- [update_automation_trigger](update_automation_trigger.rs)

## Automation templates

- [create_automation_from_template](create_automation_from_template.rs)
- [get_automation_templates](get_automation_templates.rs)

## Forms

- [delete_form](delete_form.rs)
- [find_form](find_form.rs)
- [get_form_subscribers](get_form_subscribers.rs)
- [get_forms](get_forms.rs)
- [update_form](update_form.rs)

## Webhooks

- [create_webhook](create_webhook.rs)
- [delete_webhook](delete_webhook.rs)
- [find_webhook](find_webhook.rs)
- [get_webhooks](get_webhooks.rs)
- [update_webhook](update_webhook.rs)

## Imports and batching

- [create_batch](create_batch.rs)
- [get_subscriber_import_status](get_subscriber_import_status.rs)
- [import_subscribers_to_group](import_subscribers_to_group.rs)

## Reference data

- [get_campaign_languages](get_campaign_languages.rs)
- [get_timezones](get_timezones.rs)

## E-commerce shops and settings

- [count_ecommerce_shops](count_ecommerce_shops.rs)
- [create_ecommerce_shop](create_ecommerce_shop.rs)
- [delete_ecommerce_shop](delete_ecommerce_shop.rs)
- [find_ecommerce_settings](find_ecommerce_settings.rs)
- [find_ecommerce_shop](find_ecommerce_shop.rs)
- [get_ecommerce_shops](get_ecommerce_shops.rs)
- [update_ecommerce_shop](update_ecommerce_shop.rs)

## E-commerce products

- [change_ecommerce_product_status](change_ecommerce_product_status.rs)
- [count_ecommerce_products](count_ecommerce_products.rs)
- [create_ecommerce_product](create_ecommerce_product.rs)
- [delete_ecommerce_product](delete_ecommerce_product.rs)
- [find_ecommerce_product](find_ecommerce_product.rs)
- [get_ecommerce_products](get_ecommerce_products.rs)
- [import_ecommerce_products](import_ecommerce_products.rs)
- [update_ecommerce_product](update_ecommerce_product.rs)

## E-commerce categories and assignments

- [assign_categories_to_ecommerce_product](assign_categories_to_ecommerce_product.rs)
- [assign_product_to_ecommerce_category](assign_product_to_ecommerce_category.rs)
- [count_ecommerce_categories](count_ecommerce_categories.rs)
- [create_ecommerce_category](create_ecommerce_category.rs)
- [delete_ecommerce_category](delete_ecommerce_category.rs)
- [find_ecommerce_category](find_ecommerce_category.rs)
- [get_ecommerce_categories](get_ecommerce_categories.rs)
- [get_ecommerce_category_products](get_ecommerce_category_products.rs)
- [import_ecommerce_categories](import_ecommerce_categories.rs)
- [unassign_product_from_ecommerce_category](unassign_product_from_ecommerce_category.rs)
- [update_ecommerce_category](update_ecommerce_category.rs)

## E-commerce product variants

- [delete_ecommerce_product_variant](delete_ecommerce_product_variant.rs)
- [get_ecommerce_product_variants](get_ecommerce_product_variants.rs)
- [sync_ecommerce_product_variants](sync_ecommerce_product_variants.rs)
- [update_ecommerce_product_variant](update_ecommerce_product_variant.rs)

## E-commerce customers

- [count_ecommerce_customers](count_ecommerce_customers.rs)
- [create_ecommerce_customer](create_ecommerce_customer.rs)
- [delete_ecommerce_customer](delete_ecommerce_customer.rs)
- [find_ecommerce_customer](find_ecommerce_customer.rs)
- [get_ecommerce_customers](get_ecommerce_customers.rs)
- [import_ecommerce_customers](import_ecommerce_customers.rs)
- [update_ecommerce_customer](update_ecommerce_customer.rs)

## E-commerce orders

- [count_ecommerce_orders](count_ecommerce_orders.rs)
- [create_ecommerce_order](create_ecommerce_order.rs)
- [delete_ecommerce_order](delete_ecommerce_order.rs)
- [find_ecommerce_order](find_ecommerce_order.rs)
- [get_ecommerce_orders](get_ecommerce_orders.rs)
- [import_ecommerce_orders](import_ecommerce_orders.rs)
- [update_ecommerce_order](update_ecommerce_order.rs)

## E-commerce carts

- [count_ecommerce_carts](count_ecommerce_carts.rs)
- [find_ecommerce_cart](find_ecommerce_cart.rs)
- [get_ecommerce_carts](get_ecommerce_carts.rs)
- [update_ecommerce_cart](update_ecommerce_cart.rs)

## E-commerce cart items

- [count_ecommerce_cart_items](count_ecommerce_cart_items.rs)
- [create_ecommerce_cart_item](create_ecommerce_cart_item.rs)
- [delete_ecommerce_cart_item](delete_ecommerce_cart_item.rs)
- [find_ecommerce_cart_item](find_ecommerce_cart_item.rs)
- [get_ecommerce_cart_items](get_ecommerce_cart_items.rs)
- [update_ecommerce_cart_item](update_ecommerce_cart_item.rs)

## E-commerce reporting

- [get_campaign_ecommerce_orders](get_campaign_ecommerce_orders.rs)
- [get_campaign_ecommerce_stats](get_campaign_ecommerce_stats.rs)
- [get_ecommerce_shop_stats](get_ecommerce_shop_stats.rs)
- [get_subscriber_ecommerce_orders](get_subscriber_ecommerce_orders.rs)
- [get_subscriber_ecommerce_stats](get_subscriber_ecommerce_stats.rs)
