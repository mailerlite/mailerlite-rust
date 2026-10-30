# MailerLite API quirks

Collected from the official docs (`https://developers.mailerlite.com/`) while building v2. Check here
before modelling an endpoint; most decoding bugs come from one of these.

## Transport

- Base URL `https://connect.mailerlite.com/api`; `Authorization: Bearer <key>`, `Accept: application/json`,
  JSON bodies. Optional `X-Version: YYYY-MM-DD` pins the API version (`MailerLiteBuilder::api_version`).
- Rate limits: **120 requests/minute** globally; **5/minute** for import creation
  (`POST /subscribers/import`, `POST /groups/{id}/import-subscribers`, and batches made only of
  `POST api/subscribers`). `429` carries `Retry-After` (seconds), `X-RateLimit-Limit`,
  `X-RateLimit-Remaining`. The SDK surfaces these on `ApiError`; it does not retry.
- `408` and `5xx` are MailerLite-side errors.

## Error bodies

- Usual: `{"message": "...", "errors": {"field": ["..."]}}` (422 validation). Batch/import keys look like
  `requests.0.method` or `0.name`.
- Campaign update/schedule can answer `422` with a **bare JSON string** (`"Campaign is not with status
  draft"`).
- Unassigning a subscriber from a group answers `404` with `{"message": ""}`.
- All of these are handled by `ApiError::from_parts`.

## Status codes are inconsistent

| Operation | Status |
|---|---|
| Create subscriber | `201` new / `200` existing (upsert): exposed as `Upserted::created` |
| Create group, automation, e-commerce shop/category/customer/product/order/cart item | `201` |
| Create field, campaign, webhook; variants sync; all e-commerce imports | `200` |
| Assign subscriber to group | `200` or `201` |
| Delete subscriber, group, segment, field, form, campaign, automation, webhook; unassign from group | `204`, empty body |
| Delete any e-commerce resource | `200` with the deleted resource |
| E-commerce category delete, product unassign from category | `200` with the resource **without** a `data` envelope |
| Forget subscriber | `200` `{"message", "data": Subscriber}` |

`RawRequest::data` accepts both enveloped and bare bodies, so the last case needs no special handling.

## Pagination

- **Cursor** (`CursorPage`): `GET /subscribers`, `/groups/{id}/subscribers`, `/segments/{id}/subscribers`,
  `/forms/{id}/subscribers`. `meta {path, per_page, next_cursor, prev_cursor}`; pass `cursor=`.
- **Page number** (`Page`): everything else. `meta {current_page, from, last_page, links[], path,
  per_page, to, total}`.
  - `aggregations` sits in `meta` (campaigns, forms, campaign subscriber activity) but at the **top level**
    for automations. `Page` reads both.
  - Some lists omit `meta.links` (e-commerce report orders) or all of meta/links (activity log).
- **None**: timezones, campaign languages, product variants, e-commerce settings → `Vec<T>`.
- `?limit=0` returns `{"total": N}` only (subscribers and most e-commerce lists) → `count()`.
- Surprising defaults: `GET /campaigns` defaults to `filter[status]=ready`; group/segment/form
  subscriber lists default to `filter[status]=active`.

## Value types

- **Ids** are documented as numeric strings (`"31986843064993537"`, > 2^53), but e-commerce import
  responses return JSON numbers and some nested ids are ints → always `Id` (accepts both).
- **Datetimes**: mostly `YYYY-MM-DD HH:MM:SS` (no zone); imports, webhooks and some e-commerce payloads
  use ISO-8601 with microseconds → `Timestamp` keeps the raw string.
- **Money**: numbers in most endpoints, strings (`"123.4"`) in import responses and guide examples →
  `de::number` / `de::optional_number`.
- **Rates**: `{"float": 0.25, "string": "25%"}` (`Rate`) on groups, segments, forms, campaign and
  automation stats; plain numbers on subscribers (`open_rate: 7.69`).
- **Empty objects** come back as `[]` (activity `properties`, form `settings`) → `de::object`.
- Campaign `language_id` is a string, automation email `language_id` an int.
- Automation activity uses camelCase keys (`stepRuns`, `nextStep`, `currentStep`).
- Subscriber default field keys: `name, last_name, company, country, city, phone, state, z_i_p`
  (some payloads say `zip`); treat `fields` as an open map.

## Semantics worth documenting on the method

- `POST /subscribers` only **adds** groups; `PUT /subscribers/{id}` **replaces** group membership.
  `PUT` cannot reactivate unsubscribed/bounced/junk subscribers.
- `GET /subscribers/{id}` and `DELETE /subscribers/{id}/groups/{gid}` accept an email in place of the id.
- Campaign update only works on drafts; cancel only on `ready` campaigns (400 otherwise).
- Webhooks: `batchable: true` is required for `subscriber.deleted`, `campaign.open`, `campaign.click`;
  the `Signature` header is `hex(HMAC-SHA256(raw_body, secret))` (`webhook_signature::verify`).
- Batch: max 50 requests, `path` must start with `api/`, webhook endpoints aren't allowed, a failed item
  doesn't fail the batch.
- E-commerce: `?with_resource_id` makes every id in the path *and* body your platform's `resource_id`.
  Carts are created only via orders and can't be deleted. Variant `POST` is a **sync**: if any item has a
  `resource_id`, existing variants not listed are deleted. Order `pending` triggers abandoned-cart
  automations, `complete` triggers purchase automations.

## Not in the public docs

Kept from v1 or mentioned elsewhere, but undocumented upstream. Return `serde_json::Value` and say so:
`GET /automations/{id}/monthly-totals/ecommerce`, `/automations/{id}/emails/{email_id}/ecommerce-overview-stats`,
`/automations/{id}/order-activity`. Not implemented (no REST docs): segment/form creation, single segment
fetch, automation update/enable, the `POST /subscribers/import` body, e-commerce settings update.
