# Contributing

Contributions to the MailerLite Rust SDK are welcome: bug fixes, API coverage,
tests, documentation, and examples. This guide describes how to develop and
review changes against the current 2.x codebase.

Start with the [README](README.md) for the public API and [UPGRADE.md](UPGRADE.md)
for changes from 1.x. The SDK is a single asynchronous crate targeting the current
MailerLite REST API. Classic API, blocking clients, and browser/WASM support are
outside its current scope.

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
  behavior remain in `retry.rs` and `rate_limit.rs`.
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

Return `Result` for recoverable failures. Do not introduce `unwrap`, `expect`, or
panics into normal request handling. Tests can use assertions and unwrap known
fixtures. Unsafe code is forbidden. Document every new public type, variant,
field, and method, including meaningful limits and failure behavior.
Start each implementation file with a short `//!` summary of its responsibility.

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

When adding a dependency, explain its purpose, feature impact, and Rust 1.88
compatibility in the pull request. Keep optional functionality optional. Do not
raise the MSRV or change default features incidentally.

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
   automatically. Honor `Retry-After` within the operation deadline. Do not
   assume every GET is free of side effects.
5. Use the default `ApiVersion::Latest` policy, which omits `X-Version`, unless
   the caller explicitly pins a version. Resource code must not override the
   configured version. Preserve connection reuse, pacing, and credential
   redaction through the shared transport.
6. Support only established continuation mechanisms. Rebuild continuation
   requests against the configured endpoint instead of following arbitrary
   response URLs. Subscriber activity has no reliable automatic traversal;
   keep it manual until the backend contract changes.
7. Add deterministic tests for the request and representative responses, then
   update rustdoc and an example. Update [UPGRADE.md](UPGRADE.md) for breaking
   changes and document unsupported behavior explicitly.

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

Format code, lint all targets and features, run the suite, compile examples, and
build documentation:

```sh
cargo +stable fmt --all
cargo +stable fmt --all -- --check
cargo +stable clippy --all-targets --all-features -- -D warnings
cargo +stable test --all-features
cargo +stable check --examples --all-features
RUSTDOCFLAGS='-D warnings' cargo +stable doc --no-deps --all-features
git diff --check
```

`cargo test` includes doctests; use `cargo test --doc --all-features` to run them
separately. Markdown code outside the crate's rustdoc inputs is not automatically
tested: compile changed guide snippets in a temporary consumer when appropriate.
Compiling examples must not execute their API calls.

For changes to shared code, dependencies, feature flags, or public types, verify
the supported feature combinations on stable and the MSRV. Maintainers should
run this matrix before release:

```sh
for sdk_toolchain in stable 1.88.0; do
  cargo +"$sdk_toolchain" test
  cargo +"$sdk_toolchain" check --examples
  cargo +"$sdk_toolchain" test --all-features
  cargo +"$sdk_toolchain" check --examples --all-features
  cargo +"$sdk_toolchain" test --no-default-features --features native-tls
  cargo +"$sdk_toolchain" check --examples --no-default-features --features native-tls
  cargo +"$sdk_toolchain" test --no-default-features
  cargo +"$sdk_toolchain" check --examples --no-default-features
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

Preserve nonduplicate examples, including the original v1 operation filenames.
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
change under semantic versioning. Other types do not imply a version bump by
themselves. [Release Please](https://github.com/googleapis/release-please#how-should-i-write-my-commits)
uses the commit history to prepare releases. Before a squash merge, make sure
the final commit retains the correct type and any breaking-change footer.
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

The current [workflow](.github/workflows/main.yaml) runs on pushes:

- Build and test with default features on stable, beta, and nightly.
- Check formatting and run Clippy across all targets and features on stable.
- On `main`, run release-please after build/test and quality checks pass.
- When release-please creates a release, run the crates.io publish job using the
  configured environment and repository credentials.

It does not currently have a `pull_request` trigger or dedicated MSRV,
feature-matrix, rustdoc, advisory-audit, or package-consumer jobs. Those local
checks remain important; do not assume a green workflow covers them.

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
