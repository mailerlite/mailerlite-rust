# Changelog

## Unreleased

- Migrate simple-editor emails, brand styles, and media files to the v2 resource/model layout and shared transport.
- Preserve explicit idempotency keys, replay metadata, and redacted version-conflict diagnostics.
- Add typed simple-editor campaign creation and migrate endpoint examples and regression tests.

## [2.0.0](https://github.com/mailerlite/mailerlite-rust/compare/v1.1.0...v2.0.0) (2026-10-31)


### ⚠ BREAKING CHANGES

* **resources:** Group and field list sort methods now accept SortDirection instead of a boolean. Replace false with SortDirection::Ascending and true with SortDirection::Descending.
* **models:** make evolving response models non-exhaustive
* **sdk:** Replace singular v1 accessors and string-keyed request containers with plural resource APIs, typed parameters and responses, and Result-returning client construction. Automatic retries are disabled by default.

### Features

* **audience:** add typed subscriber and audience resource APIs ([02539e2](https://github.com/mailerlite/mailerlite-rust/commit/02539e2bfef0d0eef47842624b703752bb4d1b1f))
* **automations:** add typed workflow configuration and report models ([b2685ea](https://github.com/mailerlite/mailerlite-rust/commit/b2685ea7dc81d0528c69ca28de1676b56b4817e6))
* **automations:** implement typed workflow operations ([8798f41](https://github.com/mailerlite/mailerlite-rust/commit/8798f41834f5eab988835c0662d54cc0840d42d8))
* **campaigns:** add typed campaign creation and reporting APIs ([449f8bc](https://github.com/mailerlite/mailerlite-rust/commit/449f8bc2531aad6238589cfa5d8d379300297b7f))
* **ecommerce:** implement shop-scoped resource APIs ([b7fda36](https://github.com/mailerlite/mailerlite-rust/commit/b7fda363c0d23fdfcf150c8b470436aabfb5928f))
* **ecommerce:** model shops, catalog, orders, and reports ([4cc8be5](https://github.com/mailerlite/mailerlite-rust/commit/4cc8be5e51a28624cd6c70ef3c66932b3d0106e9))
* **imports:** add batch submission and import polling ([f74851b](https://github.com/mailerlite/mailerlite-rust/commit/f74851be89d1b1b475d7983f478c251966d9bd11))
* **models:** add shared values and core response schemas ([dbb4a04](https://github.com/mailerlite/mailerlite-rust/commit/dbb4a0433f999562bd9055319c170dc6f7f70baa))
* **models:** make evolving response models non-exhaustive ([3135a44](https://github.com/mailerlite/mailerlite-rust/commit/3135a44502b56cc6dc7c65a2b2e680f5e8233cba))
* **resources:** require explicit sort directions for groups and fields ([11c3987](https://github.com/mailerlite/mailerlite-rust/commit/11c3987d762dfb5bf69f5266bf72277a1f77125e))
* **sdk:** make public exports explicit and expose common value types ([2e6bc05](https://github.com/mailerlite/mailerlite-rust/commit/2e6bc05303301c2242fc70fa8212bcbba38a871a))
* **sdk:** replace the v1 client with the typed v2 architecture ([01635c8](https://github.com/mailerlite/mailerlite-rust/commit/01635c83f7f7d8bd2a46258f16f9bd739a441f5f))
* **transport:** centralize request policy and pagination ([a29af95](https://github.com/mailerlite/mailerlite-rust/commit/a29af9591b11beb1cde23759cbe194acf4395664))
* **webhooks:** decode deliveries and verify signatures ([1ad8970](https://github.com/mailerlite/mailerlite-rust/commit/1ad8970eddaeba7add74b023ce09f6cf2954ca82))


### Bug Fixes

* accept creation responses and optional ecommerce conversion rates ([a1a7d2c](https://github.com/mailerlite/mailerlite-rust/commit/a1a7d2c4bdef0640a7eb456428e66d1892b032fc))
* bound shared server cooldowns without shortening retry delays ([ac12816](https://github.com/mailerlite/mailerlite-rust/commit/ac128169ca7ff31f13a74793409dff8ccc8ccd20))
* **http:** include serialization and decoding in operation deadlines ([b593164](https://github.com/mailerlite/mailerlite-rust/commit/b5931646d24e8a4e56308bb64e29b0b1f301357d))
* **imports:** enforce polling deadlines for late results ([225c3a7](https://github.com/mailerlite/mailerlite-rust/commit/225c3a70b3a5a797cbca232583802ab06f08106b))
* make booking availability updates explicit and safe ([f8e2936](https://github.com/mailerlite/mailerlite-rust/commit/f8e29365f6ccd115ac1c63ed18f6d41ad7198da1))
* preserve booking availability across builder calls ([dddf041](https://github.com/mailerlite/mailerlite-rust/commit/dddf041cdfce2cade9c335b61087450184f4e015))
* preserve typed JSON decode causes and add version date traits ([3151a4f](https://github.com/mailerlite/mailerlite-rust/commit/3151a4fe10c47ca23a473a06bc4f1ee8897ef628))
* remove misleading API version constant ([88422ba](https://github.com/mailerlite/mailerlite-rust/commit/88422bad8b744bf60d8a254052700204751d2037))
* require explicit ecommerce update values and strengthen regression tests ([a66a4a7](https://github.com/mailerlite/mailerlite-rust/commit/a66a4a7773165f46fca1592c0cb55c07424a34bc))
* require explicit update inputs and assert ecommerce responses ([6d83169](https://github.com/mailerlite/mailerlite-rust/commit/6d83169df90cb25e8b8c7de46c048ff008d63190))
* **sdk:** warn when lazy builders are discarded ([de78681](https://github.com/mailerlite/mailerlite-rust/commit/de78681a6b460e2db1d1023c68014b9352fcd610))
* support complete booking availability replacement ([83b34c6](https://github.com/mailerlite/mailerlite-rust/commit/83b34c6e9bf8c0814cd40c8ab471998d452e3ec8))
* unify rate models and strengthen response assertions ([4a6fc14](https://github.com/mailerlite/mailerlite-rust/commit/4a6fc1491ea10535bb0ca32087e9a41fd04df0c1))
* validate API version dates and unify weekday parsing ([366bc23](https://github.com/mailerlite/mailerlite-rust/commit/366bc23de6b5ac51637412e890844967b3f5daa7))
* validate final builder state and refine SDK conventions ([d0f805c](https://github.com/mailerlite/mailerlite-rust/commit/d0f805ce97b0de251d883d61ad6ea877ef675be0))
* validate version inputs and simplify booking schedule conversion ([ffb60d4](https://github.com/mailerlite/mailerlite-rust/commit/ffb60d4aab822fd6b7bda9d250d01f4471d08b73))


### Performance Improvements

* **http:** share serialized request bodies across attempts ([9596e83](https://github.com/mailerlite/mailerlite-rust/commit/9596e839acc18a12ef75ccfa5c4ebea18e8845fc))

## [1.1.0](https://github.com/mailerlite/mailerlite-rust/compare/v1.0.0...v1.1.0) (2026-10-29)


### Features

* add missing subscriber, group and automation endpoints ([df1efb7](https://github.com/mailerlite/mailerlite-rust/commit/df1efb7d18a3f98dca54fe061216992a571f484a))


### Bug Fixes

* correct typos in examples and README ([a503f92](https://github.com/mailerlite/mailerlite-rust/commit/a503f92caecaad49da5fbbbb6991aea42c855271))

## 1.0.0 (2023-12-21)


### Miscellaneous Chores

* release 1.0.0 ([099c5fe](https://github.com/mailerlite/mailerlite-rust/commit/099c5fe1640d824f003a994373515ad555ea4370))
* release 1.0.0 ([429c64b](https://github.com/mailerlite/mailerlite-rust/commit/429c64bfbe91ea9afd2d7f83d2de6f7efd4cc683))
