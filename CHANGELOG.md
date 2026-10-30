# Changelog

## 2.0.0 (unreleased)

### Breaking redesign

- Keep the `MailerLite` client name, make construction fallible, and replace string-keyed request containers with typed resource APIs, request builders, and response models. Add `MailerLiteBuilder` for configuration.
- Centralize authentication, dated API versioning, bounded response decoding, timeouts, local pacing, and opt-in conservative read retries. Automatic retries are disabled by default.
- Default to `ApiVersion::Latest`, omitting `X-Version`; dated version pinning remains opt-in.
- Use a 30-second request timeout and a 60-second overall operation deadline by default; both remain configurable.
- Add structured errors, cursor/page-number pagination, optional lazy streams, safe import polling, and individually inspectable batch results.
- Rebuild resource coverage from the current public REST references and backend-verified schemas. Migrate every nonduplicate v1 example, including automation steps, triggers, templates, actions, and reports, into the typed v2 architecture.
- Require Rust edition 2024 and the MSRV declared in `Cargo.toml`; use Rustls by default with native TLS optional.
- Add deterministic HTTP contract tests, practical examples, coverage tracking, and migration guidance.
- Serialize order imports as `{"orders": [...]}` to match the backend contract; callers continue passing `Vec<CreateOrder>`.
- Replace known automation configuration, template workflow, product detail, subscriber activity, form, and import-report JSON fields with concrete schemas. Retain explicit future-extension and heterogeneous batch boundaries.
- Add typed incoming webhook events and optional `webhook-signatures` HMAC-SHA256 verification over original request bytes.
- Split contract tests into independently named resource scenarios and add schema, cancellation, rate-limit, and malformed-response checks. Mark response diagnostics as truncated when a transport byte cap or interrupted body makes them incomplete.

See [upgrade guidance](UPGRADE.md) and the [example catalog](examples/README.md). This entry describes unreleased source changes, not a published release or live-account certification.

## [1.1.0](https://github.com/mailerlite/mailerlite-rust/compare/v1.0.0...v1.1.0) (2026-10-29)


### Features

* add missing subscriber, group and automation endpoints ([df1efb7](https://github.com/mailerlite/mailerlite-rust/commit/df1efb7d18a3f98dca54fe061216992a571f484a))


### Bug Fixes

* correct typos in examples and README ([a503f92](https://github.com/mailerlite/mailerlite-rust/commit/a503f92caecaad49da5fbbbb6991aea42c855271))

## 1.0.0 (2023-12-21)


### Miscellaneous Chores

* release 1.0.0 ([099c5fe](https://github.com/mailerlite/mailerlite-rust/commit/099c5fe1640d824f003a994373515ad555ea4370))
* release 1.0.0 ([429c64b](https://github.com/mailerlite/mailerlite-rust/commit/429c64bfbe91ea9afd2d7f83d2de6f7efd4cc683))
