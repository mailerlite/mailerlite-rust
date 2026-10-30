# mailerlite-rs

Typed async Rust SDK for the MailerLite API (`https://connect.mailerlite.com/api`). Rust 2024 edition,
reqwest 0.13, serde. Published to crates.io as `mailerlite-rs`; releases are cut by release-please.

## Commands

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo build --examples
cargo +1.88 check --all-features          # MSRV = rust-version in Cargo.toml
```

All of these run in CI (`.github/workflows/main.yaml`) and must pass before committing.

## Layout

| Path | What lives there |
|---|---|
| `src/lib.rs` | Crate docs, module list, re-exports, `BASE_URL` |
| `src/client.rs` | `MailerLite` (`Arc<Inner>`, cheap `Clone`) and `MailerLiteBuilder` |
| `src/http.rs` | Private request pipeline (`call`, `RawRequest`, `Response::data`), public `MailerLite::request` escape hatch, `#[cfg(test)] mod mock` |
| `src/error.rs` | `Error`, `ApiError` (status, message, validation `errors`, `retry_after`) |
| `src/pagination.rs` | `Page<T, A>`, `CursorPage<T>`, `stream::{pages, cursor}` for `list_all` |
| `src/types.rs` | `Id`, `Timestamp`, `Rate`, `Sort`, `FieldValue`, `string_enum!`, lenient `de::*` deserializers |
| `src/reports.rs` | E-commerce report types shared by subscribers/campaigns |
| `src/<resource>.rs` | One file per API area: handle, request/query builders, models, tests |
| `src/ecommerce/` | `Ecommerce` handle, `ShopScope` (shop-scoped handles), one file per e-commerce resource |
| `tests/fixtures/` | Response examples copied verbatim from the API docs |
| `examples/` | One runnable example per resource (reads `MAILERLITE_API_KEY`) |

## Conventions

- **No panics in library code.** No `unwrap`/`expect`/`panic!`/indexing that can fail; clippy enforces it
  (tests are exempt). Every fallible path returns `crate::Result`.
- **Resource handles** are `#[derive(Clone, Copy, Debug)] struct Xs<'a> { client: &'a MailerLite }`,
  created by `impl MailerLite { pub fn xs(&self) -> Xs<'_> }` in the resource's own file.
- **Requests**: build with `self.client.call(Method::X, &["segment", id.as_ref(), ...])`. Segments are
  percent-encoded individually, so never `format!` a path. Then `.query(&q)` / `.json(&body)` and:
  `.data()` to unwrap `{"data": ..}` (also accepts a bare body), `.send()` for the whole body,
  `.send_empty()` for `204`, `.execute()` when the status matters (e.g. upsert 200 vs 201).
- **Models** (responses): `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[non_exhaustive]`,
  every field documented, `#[serde(default)]` unless the field is guaranteed, `Option` for nullable,
  `Id` for ids (accepts JSON string or number), `Timestamp` for datetimes, `Rate` for `{float,string}`,
  `de::number` for numbers that may arrive as strings, `de::object` for objects PHP sends as `[]`.
- **Enums** always via `string_enum!`: unknown values decode to `Other(String)` instead of failing.
- **Request/query types**: `#[non_exhaustive]`, required values in `new(..)`, optional values via
  `#[must_use]` chained setters, `skip_serializing_if`, `#[serde(rename = "filter[x]")]` for filters,
  `Sort<XSort>` for `sort=`/`sort=-`. Single-field bodies take `&str` and use `NameBody`.
- **Pagination**: page-number lists return `Page<T>` (+ `list_all` behind `#[cfg(feature = "stream")]`),
  subscriber listings return `CursorPage<T>`; `count()` uses `?limit=0`.
- **Method docs** start with `` `METHOD /path`: `` and mention surprising semantics (e.g. `PUT` replacing
  group membership, variant sync deleting variants).
- Keep it lean: no new dependencies without a clear reason; no retries/backoff in the client (callers
  decide using `ApiError::retry_after`).

## Testing

- Unit tests use the private mock transport (`crate::http::mock::{client, fixture}`): queue responses with
  `mock.respond(status, json)`, then assert `mock.request().assert(Method::GET, "path?query").body(&json)`.
  No sockets, no mock HTTP server.
- Every endpoint gets a test asserting method, path + query string and JSON body, and decoding a fixture
  copied verbatim from the docs (`https://developers.mailerlite.com/<path>.md` serves raw markdown).
- **The SDK always targets production.** Don't add base-URL env vars, `with_base_url` builder methods or
  mock-server harnesses. For a live check against a local MailerLite, the developer temporarily edits
  `BASE_URL` in `src/lib.rs`, runs a throwaway example themselves with their key injected, and reverts
  `BASE_URL` before committing. Delete the throwaway example afterwards.

## Secrets

Never open, print or paste `.env` files, API keys or tokens. Examples read `MAILERLITE_API_KEY` from the
environment; ask the developer to inject it (`op run` / their `.env`) and run the command themselves.
gitleaks runs as a pre-commit hook via lefthook.

## Commits and releases

- Conventional commits (`feat:`, `fix:`, `docs:`, `chore:`, `feat!:` + `BREAKING CHANGE:` footer).
  release-please derives the version and changelog from them; don't edit `version` in `Cargo.toml`
  or `CHANGELOG.md` by hand.
- Resolve the base branch with `git symbolic-ref --short refs/remotes/origin/HEAD` before branching.

## Adding or changing an endpoint

Use the `add-endpoint` skill (`.claude/skills/add-endpoint/SKILL.md`). Its `reference.md` lists the
API's quirks (inconsistent status codes, bare delete bodies, numeric ids, pagination styles).
