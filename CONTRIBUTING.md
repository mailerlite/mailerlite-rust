# Contributing

Contributions to the MailerLite Rust SDK are welcome: bug fixes, API coverage,
tests, documentation, and examples. This guide describes how to develop and
review changes against the current codebase.

Start with the [README](README.md) for the public API and [UPGRADE.md](UPGRADE.md)
for changes from 1.x. The SDK is a single asynchronous crate targeting the current
MailerLite REST API.

## Reporting issues and proposing changes

For a bug report, include the SDK version, Rust version, enabled features,
operating system, expected behavior, and a minimal reproduction. Include the
HTTP status and a sanitized response shape when relevant. Remove API tokens,
webhook signing secrets, subscriber details, and other account data before
posting logs or fixtures.

For a new endpoint or behavior change, link the official API reference and
explain the intended Rust API. Discuss substantial public API changes, new
runtime dependencies, and new platform support in an issue before building a
large implementation. Small fixes and documentation improvements can go
straight to a pull request.

## Local setup

Install Rust through rustup and fork or clone the repository. Development uses
stable Rust; the minimum supported Rust version (MSRV) is **1.88.0**, declared in
[Cargo.toml](Cargo.toml). The crate uses edition 2024.

```sh
git clone https://github.com/mailerlite/mailerlite-rust.git
cd mailerlite-rust
git switch -c fix/subscriber-validation
rustup toolchain install stable --profile minimal --component rustfmt --component clippy
rustup toolchain install 1.88.0 --profile minimal
cargo +stable build
cargo +stable test
```

Use your fork's clone URL if you do not have write access to the repository.
A working native compiler and linker are required by dependencies. On Linux,
the `native-tls` feature also requires the platform TLS development libraries
and discovery tools, commonly OpenSSL headers and `pkg-config`.

Normal tests use local HTTP mock servers. They need permission to bind loopback
ports, but no MailerLite account, API token, `.env` file, or running backend.
Dependency downloads still require network access unless already cached.

`Cargo.lock` is currently ignored by this library repository. The first Cargo
build resolves dependencies and creates a local lockfile. Do not force-add it as
part of an unrelated change. Use `--locked` when deliberately checking an
existing resolution; it is unsuitable for a clean checkout with no lockfile.
Report relevant dependency-resolution differences when investigating failures.

## Repository layout

| Location | Responsibility |
| --- | --- |
| `src/lib.rs` | Public exports and crate-level documentation |
| `src/client.rs`, `src/config.rs` | Client construction, shared state, configuration, and version policy |
| `src/http/` | Authentication, headers, URL construction, decoding, deadlines, retries, and rate limits |
| `src/resources/` | Endpoint paths, operation methods, request parameters, and resource accessors |
| `src/models/` | Response models, typed automation configuration, and shared values |
| `src/error.rs`, `src/response.rs` | Public errors, successful responses, and HTTP metadata |
| `src/pagination/` | Cursor/page metadata and optional lazy streams |
| `src/webhooks/` | Incoming delivery models and optional signature verification |
| `tests/resources/` | Resource-specific HTTP contract scenarios |
| `tests/support/`, `tests/fixtures/` | Shared mock setup and sanitized reusable payloads |
| `tests/transport.rs`, `tests/errors.rs`, `tests/pagination.rs` | Cross-resource transport, error, and continuation behavior |
| `tests/webhooks.rs` | Incoming webhook delivery and signature contracts |
| `tests/thread_safety.rs` | Compile-time client, future, and stream concurrency contracts |
| `examples/` | Standalone examples, indexed by `examples/README.md` |
| `.github/workflows/main.yaml` | Build, test, formatting, lint, and release automation |

Read the affected resource's rustdoc and tests when changing SDK behavior.
The coding conventions below describe the shared implementation boundaries.

### Organizing modules

Keep small, cohesive modules in a single `.rs` file. When a module needs
multiple files, use a directory with `mod.rs` for module documentation,
declarations, and re-exports. Group implementations by responsibility rather
than adding unrelated types to the entry point:

- `models/ecommerce/` groups products, orders, shops, and other resource models
  into separate files; `product_details.rs` holds assets and booking settings.
- `models/automation_configuration/` keeps step types in `steps.rs`, trigger
  types in `triggers.rs`, and shared values in `common.rs`.
- Private `wire.rs` modules hold serialization helpers and wire-format rules.
- `http/` separates URL construction in `endpoint.rs`, request execution in
  `execution.rs`, and typed/empty response decoding in `decoding.rs`. Request
  serialization and operation policy live in `request.rs`; retry and pacing
  behavior remain in `retry.rs` and `rate_limit.rs`. `deadline.rs` shares one
  budget from serialization through decoding, checking synchronous overruns
  explicitly before dispatch and before returning a result.
- `resources/ecommerce/` keeps shop management and nested resource accessors in
  `shops.rs`, with internal path and query state in `scope.rs`.
- `pagination/` separates cursor pages, numbered pages, and navigation links into
  `cursor.rs`, `numbered.rs`, and `links.rs`. Optional lazy traversal and stream
  validation live in `stream.rs`.
- `webhooks/` separates delivery envelopes in `payload.rs`, event dispatch and
  unknown events in `events.rs`, and subscriber, campaign, group, and automation
  payloads in their respective files. Optional signature verification stays in
  `signature.rs`. These incoming delivery models are distinct from webhook
  subscription records under `models/webhooks.rs`.

Keep implementation submodules private and re-export supported types from the
existing public module. Moving a type between files should preserve its public
import path, visibility, documentation, and serialization behavior. Avoid a new
file for every small type or helper.

Keep `mod.rs` focused on module documentation, declarations, and re-exports.
Restrict shared implementation helpers to their parent module with
`pub(super)` when sibling modules need access; do not make them part of the
public API merely to move code between files.

## Coding conventions

Prefer straightforward, idiomatic Rust with focused modules and consistent
public naming. Required request fields belong in constructors; optional fields
use consuming setters. Resource handles borrow a reusable client. Avoid extra
traits, macros, dependencies, allocations, or cloning without a concrete need.

Use explicit imports in production code, including private modules. Name the
types, functions, or modules used instead of importing `module::*`; group related
names with braces and let rustfmt handle their layout. This makes dependencies
visible and avoids pulling newly added names into scope implicitly. Inline unit
test modules may use `use super::*` to exercise their parent module.

Use named enums for arguments whose meaning is unclear at the call site, such
as `.sort(GroupSort::Name, SortDirection::Descending)`. Clearly named boolean
setters such as `.enabled(true)` can retain their boolean values.

Derive `PartialEq`, `Eq`, and `Hash` for public option enums when their values
support equality and hashing. Callers need these traits to compare options or
use them as collection keys; they cannot add the implementations downstream.
Do not invent an ordering for domain choices just to derive `Ord`.

Repeated request-builder setters replace the earlier value for that parameter.
Validate the final state in `send()` and use the same validation for streams.
Correcting one parameter must not hide an invalid value in another parameter or
an option unsupported by the endpoint. Retain typed values when validation
depends on information that would be lost in the wire representation.

Mark lazy request-builder types `#[must_use]` with a message directing callers
to `.send().await`, and client configuration builders with a message directing
callers to `.build()`. Type-level annotations also catch discarded consuming
setter chains. Async request futures and pagination streams already inherit
`must_use` from `Future` and `Stream`. Keep these warnings focused on unfinished
operations rather than applying them to every resource handle or data model.

Return `Result` for recoverable failures. Do not introduce `unwrap`, `expect`, or
panics into normal request handling. Tests can use assertions and unwrap known
fixtures. Unsafe code is forbidden. Document every new public type, variant,
field, and method, including meaningful limits and failure behavior.
Start each implementation file with a short `//!` summary of its responsibility.

Keep shared lint levels in `Cargo.toml`, including the unsafe-code prohibition.
Scope lint exceptions to the smallest practical item and include a `reason`.
Unit-test modules may allow `clippy::unwrap_used` for fixtures and assertions;
integration-test targets may do the same. Do not suppress these lints across the
library with `cfg_attr(test, ...)`, which also hides warnings in production code
when compiling unit tests. Keep `expect_used` exceptions on the test or helper
that needs them; wiremock's `.expect(count)` is not a panic-producing `expect`.
Shared test helpers may allow `dead_code` individually when only some integration
targets use them. Avoid suppressing it for the entire support module.

Keep resource code responsible for endpoint semantics and route HTTP work
through the shared client helpers. Do not create an independent reqwest client
inside a resource or duplicate authentication, retry, or response decoding logic.

The internal `request<T>` helper decodes the complete wire envelope; `data<T>`
unwraps a standard `{ "data": T }` envelope; `empty` handles documented empty
successes. Choose the helper from the endpoint contract. Pagination and import
polling must use the same client so they retain authentication, deadline, and
retry policy. Use `MailerLite::http_client_builder()` for injected HTTP clients;
the SDK cannot override policies on a client that has already been built.

Request and response models have separate responsibilities. Preserve exact
identifiers and decimal values. Do not assign a timezone to timestamps that lack
one. Distinguish omitted fields from explicit null where the endpoint supports
that distinction; omission does not universally mean preservation for replacement
operations. Known discriminators should select typed variants. Malformed known
payloads must fail decoding; future variants and genuinely open fields need
explicit extension boundaries.

Treat server-owned resource, report, activity, webhook-delivery, and enriched
response configuration structs as extensible records: use `#[non_exhaustive]`
so new fields do not require downstream struct literals to change. Evolving
response enums need the same annotation; apply it to struct-like response
variants when their fields can grow. Retain explicit unknown-value variants
where supported by the wire contract. The attribute alone does not make Serde
accept unknown discriminators or malformed fields.

Keep request parameter/configuration models and finite representation enums
constructible and exhaustively matchable. These include `Patch`, `FieldValue`,
configuration IDs/scalars, weekdays, and array-versus-map representations.
Also retain construction of reusable envelopes (`Response`, pages, batch
results), metadata, import receipts, and fixed scalar wrappers such as rates
and counts. Shared `RelatedObject` remains constructible because request
conditions use it. Evolve those closed contracts deliberately rather than
adding fields or variants as routine compatible changes.

For non-exhaustive response fixtures, deserialize small sanitized JSON values;
read fields directly or destructure with `..`. Check construction and matching
contracts from a downstream crate or rustdoc: unit tests inside the defining
crate do not enforce `non_exhaustive` restrictions. Adding the attribute to an
already published type is itself a source-breaking change; this policy is part
of the 2.0 redesign and must not be applied casually in a minor release.

When adding a dependency, explain its purpose, feature impact, and Rust 1.88
compatibility in the pull request. Keep optional functionality optional. Do not
raise the MSRV or change default features incidentally.

### Public exports and dependency types

Use explicit names in public re-exports (`pub use module::{Type, OtherType}`),
including re-exports from private implementation modules. Preserve existing
public import paths when reorganizing files. Adding a public item to an
implementation module must not silently export it through a wildcard.

Define shared wire representations once and re-export them where existing public
paths need to remain available. Verify that their wire types and units agree
before consolidating them; similar field names alone do not establish the same
contract. For example, `models::common::Rate` stores a fractional `float` and a
formatted percentage, while automation report rates contain only a string.

The SDK intentionally exposes these dependency types and traits:

| Dependency family | Public use | Convenience export |
| --- | --- | --- |
| `rust_decimal` 1 | `Decimal` in monetary values and booking durations | `mailerlite_rs::Decimal` |
| `reqwest` 0.13 | `Client` injection, `ClientBuilder`, transport `Error`, and `StatusCode` | `mailerlite_rs::StatusCode` |
| `serde` 1 | `Serialize`/`Deserialize` implementations and generic serialization/decoding bounds | Use `serde` directly |
| `serde_json` 1 | `Value`, `Number`, and serialization/decoding `Error` | Use `serde_json` directly |
| `futures-core` 0.3 | Returned `Stream` implementations with the `stream` feature | Use `futures_core::Stream` directly |

Re-export common money and status values without wrapping or converting them.
Keep broader HTTP, JSON, and trait APIs under their upstream crate names. Users
can configure an injected client through `MailerLite::http_client_builder()`
without naming reqwest types. Applications using Serde derives, `json!`, or
`TryStreamExt` should declare compatible direct dependencies; the latter comes
from `futures-util` 0.3. Tokio 1 is the required runtime, rather than an exported
handle type.

Treat incompatible changes to exposed dependency types as SDK API changes even
when a signature's printed names stay the same. Re-exports preserve upstream
type identity; they do not hide dependency compatibility requirements. Review
the affected public signatures, trait bounds, and features before updating those
dependencies. Check downstream compilation with compatible direct dependencies
and with only the SDK dependency; internal unit tests alone cannot verify the
consumer's import paths. Update this table when the public boundary changes.

## Adding or changing an API operation

1. Establish the method, path, parameters, response envelope, success statuses,
   errors, and pagination contract from the
   [official reference](https://developers.mailerlite.com/getting-started).
   For additional backend-derived operations, identify the route, validator,
   and response schema used as evidence in the pull request. Contributors do
   not need a private backend checkout to work on the SDK. Existing methods or
   sample payloads alone do not establish an undocumented guarantee.
2. Cite contract sources in the pull request and relevant rustdoc or tests.
   If sources disagree, document the chosen behavior and its limitations beside
   the operation. Do not silently invent endpoints, fields, idempotency
   guarantees, or pagination behavior.
3. Add typed request parameters and response models to the appropriate resource
   and model modules. Match the actual envelope: data, count, numbered page,
   cursor page, batch result, import receipt, or empty response. A DELETE can
   return a resource; a successful HTTP response can represent pending work.
4. Classify retry safety explicitly. Retries are disabled by default. Opt-in
   retries apply only to reviewed reads; writes and batches are never replayed
   automatically. Automatic retries must honor the full `Retry-After` within
   the operation deadline. Shared server cooldowns are capped at five minutes
   per response so excessive values cannot permanently block later operations;
   preserve the original delay in response metadata. Do not assume every GET
   is free of side effects.
5. Use the default `ApiVersion::Latest` policy, which omits `X-Version`, unless
   the caller explicitly pins a version. Resource code must not override the
   configured version. Preserve connection reuse, pacing, and credential
   redaction through the shared transport.
6. Support only established continuation mechanisms. Rebuild continuation
   requests against the configured endpoint instead of following arbitrary
   response URLs. Subscriber activity has no reliable automatic traversal;
   keep it manual until the backend contract changes.
7. Add deterministic tests for the request and representative responses. Include
   a compilable rustdoc example and an error contract for the operation and its
   list/stream builders, following [Documentation and examples](#documentation-and-examples).
   Update the standalone example and [UPGRADE.md](UPGRADE.md) when applicable,
   and document unsupported behavior explicitly.

## Testing

Keep each operation or scenario independently named. Resource tests are modules
under `tests/resources/`, loaded by the `tests/resources.rs` integration target.
Ecommerce HTTP scenarios have a nested resource directory. Keep model decoding
tests beside their models in `src/models/` and request serialization or local
parameter-validation tests beside their definitions in `src/resources/`. Other
unit tests belong beside the code they exercise in `src/`. HTTP tests should
still check the decoded result and response diagnostics as part of the complete
operation.

Put focused unit tests in a `#[cfg(test)] mod tests` block at the end of the file
they exercise. For example, product decoding tests live in `products.rs`, booking
settings tests in `product_details.rs`, and step and trigger configuration tests
in their respective files. Template workflow tests belong with the template
models. Prefer this over a shared `tests.rs` collecting unrelated model tests.
Keep tests of complete public workflows in the integration suite.

`tests/thread_safety.rs` checks concurrency contracts at compile time: clients,
configuration, and error types are `Send + Sync`; representative request futures,
their outputs, and pagination streams are `Send`. It covers every stream
constructor and checks owned tasks against Tokio's `Send + 'static` bounds.
These tests construct values but never poll futures or streams, so they need no
runtime, HTTP server, or API token. Run them with and without `stream`:

```sh
cargo test --test thread_safety
cargo test --test thread_safety --all-features
```

Extend these assertions when adding a new request execution path or stream.
Test futures using SDK request models and thread-safe arguments; generic methods
can also accept caller-defined values that are not `Send`. Do not require `Sync`
or `Unpin` for futures and streams merely to support task execution. A borrowed
request need not be `'static`; test spawn compatibility by moving a client clone
into an `async move` block.

When reorganizing tests, preserve assertions and fixture provenance, update
relative `include_str!` paths, and remove obsolete module declarations and test
commands. Split mixed scenarios into independently named tests where useful;
do not duplicate coverage merely to populate a new file.

Use a fresh mock server per scenario. Assert the method, encoded path, query
parameters, relevant headers, serialized JSON, decoded result, and request count.
For local validation failures, assert that no HTTP request was made.

`tests/support::default_client` uses default version and retry policies, with
local pacing disabled for tests. `tests/support::client` explicitly enables
immediate safe-read retries. Use the latter when demonstrating that a write is
not replayed even when read retries are enabled.

Cover failure and boundary cases that apply to the change:

- Large and zero-prefixed IDs, precise money, absent fields, explicit nulls,
  empty collections, and documented alternate wire types.
- Validation details, non-JSON errors, empty responses, unexpected success
  shapes, truncated bodies, and retained HTTP metadata.
- Rate limits, bounded retries, operation deadlines, and cancellation. Prefer
  controlled time over slow or fragile wall-clock sleeps.
- Pagination filters and progress, repeated cursors, and untrusted continuation
  destinations. Verify credentials cannot be forwarded to another host.
- Redacted debug output and diagnostics. For webhook verification, preserve raw
  bytes and use independently established signature vectors.

Use inline JSON for short, operation-specific payloads. Put shared or complex
payloads in `tests/fixtures/` and load them with `include_str!` where useful.
Fixtures must be synthetic or sanitized. Do not remove a fixture without
checking its consumers. A larger test count alone does not prove better coverage.

Run focused tests while developing:

```sh
cargo +stable test --test resources subscribers::
cargo +stable test --test resources ecommerce::products:: --all-features
cargo +stable test --lib models::ecommerce:: --all-features
cargo +stable test --lib resources::ecommerce::params:: --all-features
cargo +stable test --lib models::automation_configuration:: --all-features
cargo +stable test --lib models::automation_templates:: --all-features
cargo +stable test --lib models::subscriber_activity::
cargo +stable test --lib resources::batch::
cargo +stable test --test errors
cargo +stable test --test transport
cargo +stable test --test pagination --features stream
cargo +stable test --test webhooks --features webhook-signatures
```

## Checks before a pull request

Format code, lint all targets and features, run the suite (including example
compilation), and build documentation:

```sh
cargo +stable fmt --all
cargo +stable fmt --all -- --check
cargo +stable clippy --all-targets --all-features -- -D warnings
cargo +stable test --all-features
RUSTDOCFLAGS='-D warnings' cargo +stable doc --no-deps --all-features
git diff --check
```

`cargo test` includes doctests; use `cargo test --doc --all-features` to run them
separately. Markdown code outside the crate's rustdoc inputs is not automatically
tested: compile changed guide snippets in a temporary consumer when appropriate.
Compiling examples must not execute their API calls.

For a faster check while editing standalone examples, use
`cargo +stable check --examples --all-features`. A separate example-compilation
step is unnecessary immediately after `cargo test` with the same features.

For changes to shared code, dependencies, feature flags, or public types, verify
the supported feature combinations on stable and the MSRV. Maintainers should
run this matrix before release:

```sh
for sdk_toolchain in stable 1.88.0; do
  cargo +"$sdk_toolchain" test
  cargo +"$sdk_toolchain" test --all-features
  cargo +"$sdk_toolchain" test --no-default-features --features native-tls
  cargo +"$sdk_toolchain" test --no-default-features
done
```

Default features enable Rustls. Native TLS alone requires disabling defaults.
No-default-feature tests use HTTP mocks; that configuration does not independently
provide HTTPS. The all-feature suite exercises streams and webhook signatures.
For a documentation-only change, check affected links and examples rather than
claiming to have rerun unrelated tests. Report checks actually run and any gaps.

## Documentation and examples

Keep the [README](README.md) focused on getting started. Put upgrade steps in
[UPGRADE.md](UPGRADE.md), implementation guidance in this contribution guide,
and endpoint details in resource rustdoc. Update relative links when
moving documentation. User-facing guides should describe the current SDK,
without implementation-session history or agent ownership notes.

The crate-level overview lives in `src/lib.rs`; it does not embed the README.
Use intra-doc links there to generated types, modules, and methods. Repository
guides and examples are not copied into rustdoc output, so do not link to their
relative file paths from rustdoc. Keep README links relative for GitHub and local
Markdown previews. Keep shared quick-start instructions and defaults consistent
between the README and crate overview.

### Resource rustdoc

Model field documentation should explain representations and units, not repeat
field names. Distinguish a fractional rate from its percentage display string,
MailerLite IDs from external resource IDs, and missing values from zero or false.
Verify units and meanings against the API contract or backend source before
documenting them; do not infer a timezone for an unspecified timestamp.

Every exposed endpoint operation needs an inline `# Examples` section. Public
fallible constructors, request helpers, and decoding helpers also need examples
and `# Errors`. Put the summary and important behavior first, then `# Examples`,
then `# Errors`. Link related types and methods with rustdoc links such as
``[`Self::send`]`` or ``[`crate::Error::Api`]``.

Describe the actual contract rather than repeating the method name:

- Distinguish validation performed before dispatch from validation performed by
  MailerLite. For example, invalid path IDs can fail locally while IDs in request
  bodies or query parameters may be checked only by the server.
- Identify relevant error variants and actionable details, such as HTTP 422
  validation messages. Do not claim every operation can produce every error.
- Explain writes that replace data, schedule delivery, enroll subscribers, or
  delete resources. State retry behavior where it matters, and explain when a
  failure after dispatch can leave the server-side outcome uncertain.
- Distinguish acceptance receipts from completed work and describe explicit
  polling, cancellation, and partial failures where applicable.
- For list constructors, explain that they build a request and link to the
  execution method's errors. An infallible setter does not need an artificial
  `# Errors` section; document deferred validation at `send` or consumption.
- For streams, document errors yielded as items, termination after an error,
  continuation requirements, and the scope of deadlines. Items already yielded
  remain available to the caller.

Check these claims against the implementation and established API contract.
Use [subscriber operations](src/resources/subscribers.rs),
[import polling](src/resources/imports.rs), and
[webhook verification](src/webhooks/signature.rs) as examples of the format.
Standalone files in `examples/` complement these method-level docs; they do not
replace them.

### Writing and formatting examples

Use `no_run` Rust fences for examples that could contact an API or change account
data. They must compile without an API token or running backend. Pure examples,
such as decoding synthetic data or verifying a sample signature, should run as
doctests. Use `?` for recoverable errors and keep credentials synthetic.

Rustdoc source comments may hide a wrapper function and its final `Ok(())` with
`# ` lines. Keep useful imports and operation steps visible. For feature-gated
methods, attach examples to the gated item and test with that feature enabled.
Document any application dependencies needed to consume a stream or decode JSON.
Do not use `ignore` to hide a broken example or bypass feature verification.

README and other standalone Markdown snippets are not run by `cargo test --doc`.
Compile changed snippets in a temporary consumer with their required features.
The README's webhook `receive` function also matches `examples/verify_webhook.rs`,
which is compiled with `webhook-signatures` enabled; keep the two in sync.

Separate logical steps with a blank line: configuration, parameter construction,
request execution, result handling, and return. Keep related setup variables
together, and keep fluent method chains intact. In `///` comments, use a bare
`///` line for the gap. Apply the same spacing to standalone examples and Markdown
guides:

```rust
use mailerlite_rs::MailerLite;

async fn count_subscribers(mailerlite: &MailerLite) -> mailerlite_rs::Result<()> {
    let response = mailerlite.subscribers().count().await?;

    println!("{} subscribers", response.data.total);

    Ok(())
}
```

Rustfmt does not insert these logical gaps into documentation snippets; review
their rendered appearance as well as their compilation.

Preserve nonduplicate examples, including the original v1 operation filenames.
Update examples must take replacement values from explicit input, including
names, prices, quantities, and attribute maps. Document required environment
variables and which fields are replaced in the file header. Describe fixed
operation choices, such as expressing a delay in days, in that header too.
Update their implementations when APIs change. List new examples in
[examples/README.md](examples/README.md), document required environment variables,
and declare `required-features` in Cargo.toml for examples that need optional
features. Use normal Rust functions in standalone Markdown; rustdoc hidden-line
prefixes such as `# ` are visible on GitHub.

The SDK does not load `.env` automatically. Live examples read environment
variables explicitly and may modify account data or send email. Running them
is separate from test and compilation checks. Use a suitable test account or an
explicitly configured local backend, and keep credentials out of source,
fixtures, command output, and pull requests.

### Checking rendered documentation

Run doctests with default and optional features, then build and open rustdoc:

```sh
cargo +stable test --doc
cargo +stable test --doc --all-features
cargo +1.88.0 test --doc --all-features
RUSTDOCFLAGS='-D warnings' cargo +stable doc --no-deps --all-features --open
```

The crate landing page displays the overview written in `src/lib.rs`. Method
examples and error contracts appear on their resource type's page. For example, navigate to
`resources → subscribers → Subscribers → count`, or open
`target/doc/mailerlite_rs/resources/subscribers/struct.Subscribers.html#method.count`.
Expand the method's documentation if collapsed. After changing source comments,
regenerate the docs and refresh any previously opened browser tab.

Check headings, code highlighting, blank lines, and links. Confirm that network
examples are compile-only and report any intentionally ignored examples when
summarizing verification. The commands above do not run standalone examples or
require live API credentials.

## Conventional Commits

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for
commit messages and pull request titles. The format is:

```text
<type>(<optional scope>): <short description>
```

Use a concise imperative description, such as `preserve validation details`.
Scopes should identify the affected area, for example `subscribers`, `transport`,
`pagination`, `webhooks`, or `docs`.

| Type | Use for |
| --- | --- |
| `feat` | New public capabilities |
| `fix` | Bug fixes |
| `docs` | Documentation and guides |
| `test` | Test additions or corrections |
| `refactor` | Code restructuring without an intended behavior change |
| `perf` | Performance improvements |
| `build` | Dependencies and build configuration |
| `ci` | Workflow changes |
| `style` | Formatting-only changes |
| `chore` | Other maintenance |

Examples:

```text
feat(subscribers): add a typed activity filter
fix(transport): preserve metadata on truncated responses
docs(contributing): rewrite contributor guidance
ci: add formatting and Clippy checks
```

Mark breaking changes with `!` and explain the migration in a `BREAKING CHANGE:`
footer. Update UPGRADE.md in the same pull request:

```text
feat(subscribers)!: replace create with typed upsert

BREAKING CHANGE: Replace subscriber().create(Data) with
subscribers().upsert(UpsertSubscriber::new(email)).
```

`fix` denotes a patch, `feat` a minor change, and a breaking-change marker a major
change under semantic versioning. Nonbreaking `docs`, `test`, `refactor`, `build`,
`ci`, `style`, and `chore` commits do not request a release by themselves.
[Release Please](https://github.com/googleapis/release-please#how-should-i-write-my-commits)
uses commits merged into `main` to prepare releases. When several changes are
pending, the highest required version bump wins. Before a squash merge, make
sure the final commit retains the correct type and any breaking-change footer;
individual branch commits will be replaced by that squash commit. Choose the
type that describes the change rather than labeling maintenance as `fix` just
to request a release. See [CI and releases](#ci-and-releases) for the release flow.
Conventions are reviewed by contributors and maintainers; the current workflow
does not run commitlint.

## Pull requests and review

Keep a pull request focused on one problem. Use a Conventional Commit title and
include:

- The problem and resulting behavior, with a before/after example when useful.
- API documentation or backend schema evidence for contract changes.
- Relevant tests and commands actually run, including compiler and feature
  combinations when they matter.
- Compatibility implications, documentation updates, and remaining limitations.

Separate unrelated cleanup from behavior changes. Respond to review by updating
the implementation, tests, and description together. Do not present a local
check as a GitHub-hosted CI run. Do not include tokens or real customer payloads
in screenshots, logs, or attachments.

## CI and releases

The current [workflow](.github/workflows/main.yaml) runs on branch and tag pushes:

- Run `cargo test`, which builds the library and enabled examples and runs unit,
  integration, and documentation tests, then check rustdoc
  with warnings denied on Rust 1.88.0, stable, beta, and nightly for default
  features, all features, native TLS alone, and no default features (16 matrix
  jobs).
- Check formatting and run Clippy across all targets and features on stable.
- On `main`, run release-please after build/test and quality checks pass.

The workflow runs regardless of commit type. Release Please uses
`release-type: rust` and the commit history since the last release to prepare
the next version:

1. Merge a change into `main`. A `fix` requests a patch release, a `feat` requests
   a minor release, and a breaking-change marker requests a major release.
2. After CI passes, Release Please opens or updates a release PR with the version
   bump and changelog. Further changes can accumulate in that PR before release.
3. A maintainer reviews the release PR and completes the checks below, then
   merges it into `main` when ready to release.
4. After CI passes on that merge, Release Please creates the version tag and
   GitHub Release. Its `release_created` output enables the separate crates.io
   publish job, using the configured environment and repository credentials.

A feature or fix merge therefore prepares a release; publication follows the
release PR merge. Release Please manages the version, changelog, tag, and GitHub
Release; our publish job uploads the crate. See the upstream
[release PR lifecycle](https://github.com/googleapis/release-please#whats-a-release-pr)
and [conditional publication](https://github.com/googleapis/release-please-action#automating-publication-to-npm)
documentation for how these stages connect.

Feature and rustdoc checks live in the existing build/test job. CI compiles
examples without running them, and normal tests require no API token. It does
not currently have a `pull_request` trigger, advisory-audit, or package-consumer
job; those release checks still need to be completed locally.

Keep the full toolchain/feature matrix. Separate `cargo build` and
`cargo check --examples` steps would repeat compilation already covered by
`cargo test`; a standalone rustdoc check remains necessary for documentation
warnings and links. Release creation and publication remain restricted to `main`.

Maintainers should complete these steps before approving a release:

1. Review the release-please version and changelog, including breaking changes
   and UPGRADE.md. Check that resource rustdoc and examples match the final API.
2. Run the stable/MSRV feature matrix, formatting, Clippy, rustdoc, and example
   compilation on the intended release revision.
3. Review dependency advisories and new dependency licenses. `cargo audit`, when
   installed, checks the locally resolved Cargo.lock; record the resolution and
   findings rather than treating an old audit result as current.
4. Inspect and verify the package from a clean checkout:

   ```sh
   cargo +stable package --list
   cargo +stable package
   ```

   Confirm that source, examples, fixtures, license, and guides are included,
   while `.env`, credentials, and build output are excluded. `--allow-dirty` is
   useful for local inspection but is not evidence of a clean release revision.
5. Compile a temporary downstream crate against the extracted package under
   `target/package/`, using its actual versioned directory. Check both stable and
   Rust 1.88 with the required optional features; compilation needs no API token.
6. Confirm the release workflow targets the intended source revision and that
   the release environment is configured. The publish job uses `--no-verify`,
   so package verification must be completed beforehand.

Version bumps, release tags, and publication belong to the maintainer release
process. A normal contribution should not manually publish the crate or create
release tags. Local package checks do not publish anything.
