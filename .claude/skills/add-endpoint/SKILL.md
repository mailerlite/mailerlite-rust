---
name: add-endpoint
description: Add or change a MailerLite API endpoint in the mailerlite-rs SDK — model the request/response types, add the method on the resource handle, write mock-transport tests with doc fixtures, update examples/README, and run the full check suite. Use when asked to add, fix, or update an endpoint, model field, filter, or resource in this crate.
---

# Add or change an endpoint

Work through these steps in order. Read `CLAUDE.md` first if you haven't in this session, and
`reference.md` (next to this file) for the API's quirks.

## 1. Read the official docs for the endpoint

- Raw markdown is served at `https://developers.mailerlite.com/<path>.md`, e.g.
  `api/subscribers.md`, `ecommerce/orders.md` (index: `https://developers.mailerlite.com/llms.txt`).
- Note for each endpoint: method + path, query params (names, enums, defaults, required), body fields
  (types, required, nesting), **success status code**, response shape (`data` envelope or bare,
  pagination style, `aggregations`), and documented errors.
- Anything the docs don't state is UNVERIFIED: make it `Option`/`#[serde(default)]`, and say so in the
  doc comment rather than guessing.

## 2. Find the home

- Existing area → its file in `src/` (or `src/ecommerce/`). New area → new `src/<area>.rs`, add
  `pub mod <area>;` to `src/lib.rs`, list the accessor in the crate docs' "Resources" section.
- Reuse before adding: `crate::types` (`Id`, `Timestamp`, `Rate`, `Sort`, `FieldValue`, `NameBody`,
  `COUNT_QUERY`/`Total`, `de::*`), `crate::reports`, `crate::pagination`, and models from sibling
  modules (e.g. `subscribers::Subscriber`, `groups::Group`, `campaigns::Email`).

## 3. Model the types

Copy the shape of the nearest existing type (`src/subscribers.rs` and `src/groups.rs` are the
reference implementations).

- Response model: `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[non_exhaustive]`, doc
  every field, `#[serde(default)]` unless guaranteed, `Option` for nullable, `Id`/`Timestamp`/`Rate`,
  `de::number` for numbers that may be strings, `de::object` for `{}`-or-`[]`, `de::null_default` for
  collections/counts that may be `null`.
- Enums: `string_enum! { /// doc \n pub enum X { /// doc \n Variant => "wire", ... } }`.
- Query: `#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)] #[non_exhaustive]`, fields
  `Option<_>` with `skip_serializing_if`, filters renamed `filter[...]`, `Sort<XSort>`, plus `new()` and
  `#[must_use]` setters.
- Body: `#[non_exhaustive]`, required values as `new(..)` args, optional ones as `#[must_use]` setters
  and `skip_serializing_if`. If "empty" and "absent" mean different things (e.g. `PUT` replacing
  `groups`), use `Option<Vec<_>>` so `Some(vec![])` is expressible.

## 4. Add the method

```rust
/// `GET /things/{id}/widgets`: what it returns, and any surprising semantics.
pub async fn widgets(&self, id: impl AsRef<str>, query: &ListWidgets) -> Result<Page<Widget>> {
    self.client
        .call(Method::GET, &["things", id.as_ref(), "widgets"])
        .query(query)
        .send()
        .await
}
```

- Pick the terminal call by response shape: `.data()` (`{"data": T}` or bare `T`), `.send()` (whole
  body: lists, `{"total"}`, custom envelopes), `.send_empty()` (`204`), `.execute()` when the status
  code carries meaning (then `response.status` + `response.data()?`).
- Lists: also add `list_all` behind `#[cfg(feature = "stream")]` using `pagination::stream::paged`
  (page-number lists; add the query type to the `paged_query!` list in `src/pagination.rs`) or
  `::cursored` (subscriber listings). See `Groups::list_all` / `Subscribers::list_all`. Both helpers
  stop on repeated cursors or pages, so never hand-roll a pagination loop.
- Never build paths with `format!`; pass segments so ids/emails are percent-encoded.

## 5. Test it

In the module's `#[cfg(test)] mod tests`:

1. Copy the docs' response example verbatim to `tests/fixtures/<area>_<name>.json`.
2. Add a `#[tokio::test]` that queues it with `mock.respond(status, fixture("..."))`, calls the method
   with every param/body field you added, asserts `mock.request().assert(Method::X, "path?query")`
   (query is percent-encoded: `filter%5Bname%5D=...`) and `.body(&json!({...}))`, and checks a few
   decoded fields, including any lenient ones.
3. For new enums/lenient fields, assert the fallback path too (`Other(..)`, string money, `[]` objects).

## 6. Document

- Doc comment on every public item (`missing_docs` is on). Add a `no_run` example to new handles or
  non-obvious builders.
- Update `examples/<area>.rs` if the endpoint is part of a common flow, and the resource table in
  `README.md` when adding an area. Mention breaking changes in `MIGRATION.md`.

## 7. Verify

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo build --examples
```

Optional live check (only if the developer asks): write a throwaway `examples/live_check.rs`, ask them to
run it with their key injected (the SDK targets production; for a local API they temporarily edit
`BASE_URL` themselves), fix mismatches, then delete the example and make sure `BASE_URL` is reverted.
Never read `.env` files or print keys.

## 8. Commit

Conventional commit: `feat: add <endpoint>` / `fix: <model> field <x> may be null`. A change that breaks
the public API needs `feat!:` and a `BREAKING CHANGE:` footer.
