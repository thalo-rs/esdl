# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.6.0](https://github.com/thalo-rs/esdl/compare/0.5.0..0.6.0) - 2022-12-12
#### Bug Fixes
- duplicate error type in rust codegen - ([6d71a16](https://github.com/thalo-rs/esdl/commit/6d71a16aa8699feb5658ded9b7bc006f927f2c59)) - [@tqwewe](https://github.com/tqwewe)
#### Documentation
- add git dependency note to README.md - ([3b4dc52](https://github.com/thalo-rs/esdl/commit/3b4dc5206e54b84d0165c56a6b4b2748e5fd1231)) - [@tqwewe](https://github.com/tqwewe)
#### Features
- implement `Eq` for `Schema` types - ([8601cce](https://github.com/thalo-rs/esdl/commit/8601cce3336a772fd8fecfee84460830d7ed4ccf)) - [@tqwewe](https://github.com/tqwewe)
- add `Long`, `Double`, `Bytes` and remove `Timestamp` and `UInt` - ([28c4666](https://github.com/thalo-rs/esdl/commit/28c466662bc6e7f8a59f46e6b87b330ffa360283)) - [@tqwewe](https://github.com/tqwewe)
- pest schema definition - ([7907d00](https://github.com/thalo-rs/esdl/commit/7907d000112248db4e1e187b15a7f93d50b8bf98)) - [@tqwewe](https://github.com/tqwewe)
- support for `UInt` type - ([b21106d](https://github.com/thalo-rs/esdl/commit/b21106dfe1571a687d6f39f23f582881be1be478)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- remove unused wasm feature flag - ([8f3540b](https://github.com/thalo-rs/esdl/commit/8f3540bf3fcc6c0469bd69b8fcc5b4046fd625bc)) - [@tqwewe](https://github.com/tqwewe)
#### Refactoring
- remove all codegen - ([1e07ae9](https://github.com/thalo-rs/esdl/commit/1e07ae9a3ee590cf57c9bc70ed57f9225e4017f4)) - [@tqwewe](https://github.com/tqwewe)

- - -

## [0.5.0](https://github.com/thalo-rs/esdl/compare/0.4.0..0.5.0) - 2022-05-15
#### Bug Fixes
- **(examples/bank-account-wasm)** build script - ([214d730](https://github.com/thalo-rs/esdl/commit/214d7307bbb63d62e0195f45c1108995e22f413e)) - [@tqwewe](https://github.com/tqwewe)
- rust codegen derive hash for f64 - ([0c8497c](https://github.com/thalo-rs/esdl/commit/0c8497cd9e12ea3d41348f892609124e48d851be)) - [@tqwewe](https://github.com/tqwewe)
- cfg syntax error - ([924c466](https://github.com/thalo-rs/esdl/commit/924c4662f7b808371544e2f92346fedf4c0e555b)) - [@tqwewe](https://github.com/tqwewe)
#### Documentation
- fix version example in README.md - ([1b4c553](https://github.com/thalo-rs/esdl/commit/1b4c553f8c1e2b33df722d8978801a988285beb2)) - [@tqwewe](https://github.com/tqwewe)
- add command and event wasm format docs - ([e217c6c](https://github.com/thalo-rs/esdl/commit/e217c6c58ad25a02c7393e12e83c42443a3bda12)) - [@tqwewe](https://github.com/tqwewe)
- fix code example in examples/bank-account-wasm README.md - ([11e82da](https://github.com/thalo-rs/esdl/commit/11e82da5c0bb233ba144f4d99c35bf554a68ca99)) - [@tqwewe](https://github.com/tqwewe)
- add version to README example - ([16c9477](https://github.com/thalo-rs/esdl/commit/16c94772c0ebc83037242d2427baf71b81ab9973)) - [@tqwewe](https://github.com/tqwewe)
- update README with optional syntax - ([6e63a97](https://github.com/thalo-rs/esdl/commit/6e63a971bfff162296598131a2210b908273a8a5)) - [@tqwewe](https://github.com/tqwewe)
#### Features
- **(examples/bank-account-wasm)** update schema to use new optional syntax - ([a7045c1](https://github.com/thalo-rs/esdl/commit/a7045c13deb6ef671a77c3ea821cb01fb7820afc)) - [@tqwewe](https://github.com/tqwewe)
- add version support - ([48575f1](https://github.com/thalo-rs/esdl/commit/48575f1cf6638c66f2c311787c5198e2eeedbb03)) - [@tqwewe](https://github.com/tqwewe)
- add additional dynamic derives to rust codegen - ([842b5f2](https://github.com/thalo-rs/esdl/commit/842b5f26714b95c27f2d58dafa04f5771a3b800f)) - [@tqwewe](https://github.com/tqwewe)
- implement `serde::Deserialize` for schema types - ([eb6425f](https://github.com/thalo-rs/esdl/commit/eb6425f2624db7b0860428b9a9003d71dbba6543)) - [@tqwewe](https://github.com/tqwewe)
- use `?` for optional types - ([8029e59](https://github.com/thalo-rs/esdl/commit/8029e59552066ed093e80bb0919320651159d251)) - [@tqwewe](https://github.com/tqwewe)
- use `->` separator for return type - ([9b33cf7](https://github.com/thalo-rs/esdl/commit/9b33cf79d32822d8402f3ec699f097c19eb99069)) - [@tqwewe](https://github.com/tqwewe)
- add codegen-rust-wasm - ([288ac15](https://github.com/thalo-rs/esdl/commit/288ac15b41ad876159aea70a76dbd9ff2e2d8f47)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- merge pull request #9 from thalo-rs/feat/versioning - ([bdbec86](https://github.com/thalo-rs/esdl/commit/bdbec864e8eccc7de5d4e1f3ea4496e382e9e662)) - [@tqwewe](https://github.com/tqwewe)
- upgrade git versions in Cargo.lock - ([b04168c](https://github.com/thalo-rs/esdl/commit/b04168ce7cbb9f6c462147c5bcaba70b3eca0b85)) - [@tqwewe](https://github.com/tqwewe)
- merge branch 'main' into feat/versioning - ([2f8400c](https://github.com/thalo-rs/esdl/commit/2f8400cb97a257a882192c8ceb37a6e4d133c075)) - [@tqwewe](https://github.com/tqwewe)
- upgrade dependencies - ([0080411](https://github.com/thalo-rs/esdl/commit/008041131bce9ed139202178a45ce20da9056ac5)) - [@tqwewe](https://github.com/tqwewe)
- merge pull request #8 from thalo-rs/feat/explicit-optional - ([ff217f1](https://github.com/thalo-rs/esdl/commit/ff217f11a50e33f6b9a1a23b80bfe27a40eb8770)) - [@tqwewe](https://github.com/tqwewe)
- merge branch 'main' into feat/explicit-optional - ([4bbb66d](https://github.com/thalo-rs/esdl/commit/4bbb66db827b99c801bb25d9059e0cbcd9e3f152)) - [@tqwewe](https://github.com/tqwewe)
- merge branch 'main' of github.com:thalo-rs/esdl into feat/syntax-updates - ([80f975e](https://github.com/thalo-rs/esdl/commit/80f975ecc17bdd4f4e94824b5b5d22b5df05d938)) - [@tqwewe](https://github.com/tqwewe)
#### Refactoring
- modules and errors - ([652f8c7](https://github.com/thalo-rs/esdl/commit/652f8c7c94cfcb96b4b772486f62503bdfacd5f0)) - [@tqwewe](https://github.com/tqwewe)
#### Tests
- add schema parse tests - ([ece94a9](https://github.com/thalo-rs/esdl/commit/ece94a96c67d01740d8efb6505874daf71f5f67c)) - [@tqwewe](https://github.com/tqwewe)
- - -

## [0.4.0](https://github.com/thalo-rs/esdl/compare/0.3.0..0.4.0) - 2022-01-30
#### Bug Fixes
- codege-typescript `compile_before` implementation - ([3811495](https://github.com/thalo-rs/esdl/commit/3811495a32bda465ad5579b8d23650577bf10c6d)) - [@tqwewe](https://github.com/tqwewe)
#### Documentation
- **(examples/bank-account-wasm)** add bank-account-wasm example - ([694a050](https://github.com/thalo-rs/esdl/commit/694a0501399f2103023ef15ee68b52e520d4fea3)) - [@tqwewe](https://github.com/tqwewe)
#### Features
- add wasm feature flag and code generation - ([42ab3f9](https://github.com/thalo-rs/esdl/commit/42ab3f91f3177133462aeaead522a33f118021c7)) - [@tqwewe](https://github.com/tqwewe)
- add rust codegen command enum - ([0c53833](https://github.com/thalo-rs/esdl/commit/0c5383372973c4795bcfad12863af972c8cf484f)) - [@tqwewe](https://github.com/tqwewe)
- serde rename codegen event enum - ([f53e950](https://github.com/thalo-rs/esdl/commit/f53e950064e6f41b803ffe59f32d61f4309fbe25)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- Cargo.lock update - ([5cbc0cf](https://github.com/thalo-rs/esdl/commit/5cbc0cf2c01abfbfd020406e46e79daed999f405)) - [@tqwewe](https://github.com/tqwewe)
- - -

## [0.3.0](https://github.com/thalo-rs/esdl/compare/0.2.1..0.3.0) - 2022-01-28
#### Documentation
- simplify chrono `DateTime` - ([a195013](https://github.com/thalo-rs/esdl/commit/a1950133138154b17548db0257c574379381c524)) - [@tqwewe](https://github.com/tqwewe)
- clarify types in README.md - ([15149fc](https://github.com/thalo-rs/esdl/commit/15149fcacb6ec03ee19aae64887614afaa20a273)) - [@tqwewe](https://github.com/tqwewe)
- add TypeScript types and links to README.md - ([cc68770](https://github.com/thalo-rs/esdl/commit/cc68770d13369e1cf51313b44cd12fb37128bfea)) - [@tqwewe](https://github.com/tqwewe)
- add TypeScript types and links to README.md - ([727d5b1](https://github.com/thalo-rs/esdl/commit/727d5b1940e1a2d01aa2906ccdf6f0915c1acb6b)) - [@tqwewe](https://github.com/tqwewe)
#### Features
- add support for TypeScript codegen - ([0cd7e99](https://github.com/thalo-rs/esdl/commit/0cd7e99256f13ab699efd8c12f06cf318e2cfc97)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- merge pull request #2 from thalo-rs/feat/codegen-ts - ([3a26edd](https://github.com/thalo-rs/esdl/commit/3a26eddc1ed327dac6544ebafc6e82abae833cc7)) - [@tqwewe](https://github.com/tqwewe)
- add keywords and categories to Cargo.toml - ([2ac41c0](https://github.com/thalo-rs/esdl/commit/2ac41c03a9b187ed17938177d925a5945a3d9b2b)) - [@tqwewe](https://github.com/tqwewe)
#### Refactoring
- move rust codegen into separate module - ([fd41f01](https://github.com/thalo-rs/esdl/commit/fd41f01ba34342797c3d0b81b3627a9db8e0d4f1)) - [@tqwewe](https://github.com/tqwewe)
- - -

## [0.2.1](https://github.com/thalo-rs/esdl/compare/0.2.0..0.2.1) - 2022-01-27
#### Features
- implement `Serialize` and `Deserialize` on type structs - ([97de0a0](https://github.com/thalo-rs/esdl/commit/97de0a01e097f64f1fcd778d7c6ac735439ee6fb)) - Mike Shearer
#### Refactoring
- run cargo fmt - ([d586634](https://github.com/thalo-rs/esdl/commit/d586634bade768fb6c818a79b3d15db3d3fac876)) - [@tqwewe](https://github.com/tqwewe)
- - -

## [0.2.0](https://github.com/thalo-rs/esdl/compare/0.1.0..0.2.0) - 2022-01-25
#### Features
- add parsing options for `Schema` - ([921b3c7](https://github.com/thalo-rs/esdl/commit/921b3c77349c5038336e29ff74cbacc6f9ae1242)) - [@tqwewe](https://github.com/tqwewe)
- add serde derives to schema - ([8914873](https://github.com/thalo-rs/esdl/commit/891487365450b0d845f2bf46f597651baead507a)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- add CHANGELOG.md - ([5c1abc4](https://github.com/thalo-rs/esdl/commit/5c1abc4035a468ddbb218805466c2ff5a0114488)) - [@tqwewe](https://github.com/tqwewe)
- add bump books to cog.toml - ([195c86c](https://github.com/thalo-rs/esdl/commit/195c86c6d64bc97232a81d92ec3997448c39c579)) - [@tqwewe](https://github.com/tqwewe)
- revert "chore: rename to esl" - ([15ad3da](https://github.com/thalo-rs/esdl/commit/15ad3da39ceac64a09d1730ba3faad8d8a42cf0f)) - [@tqwewe](https://github.com/tqwewe)
- rename to esl - ([b0a3d21](https://github.com/thalo-rs/esdl/commit/b0a3d21a70ff56737d513ffcf58150c0eaf70b43)) - [@tqwewe](https://github.com/tqwewe)
- - -

## [0.1.0](https://github.com/thalo-rs/esdl/compare/52dd3bbf938b81e6ff8e5b99b6d84243ecf9fcf8..0.1.0) - 2022-01-25
#### Bug Fixes
- event suffix in command events - ([e240973](https://github.com/thalo-rs/esdl/commit/e24097328b5371da9d37115220867388236d1937)) - [@tqwewe](https://github.com/tqwewe)
- incorrect reference to event in return type - ([8ef5eec](https://github.com/thalo-rs/esdl/commit/8ef5eecf17ddba05c3bc930112740156dcfc8e13)) - [@tqwewe](https://github.com/tqwewe)
#### Documentation
- add scalar types and syntax to README.md - ([3e2e90d](https://github.com/thalo-rs/esdl/commit/3e2e90d3f243ec995b6a7d1146a14b4c93bd94b0)) - [@tqwewe](https://github.com/tqwewe)
#### Features
- add parsing options for `Schema` - ([921b3c7](https://github.com/thalo-rs/esdl/commit/921b3c77349c5038336e29ff74cbacc6f9ae1242)) - [@tqwewe](https://github.com/tqwewe)
- add serde derives to schema - ([8914873](https://github.com/thalo-rs/esdl/commit/891487365450b0d845f2bf46f597651baead507a)) - [@tqwewe](https://github.com/tqwewe)
- add scalar `Timestamp` - ([722af3c](https://github.com/thalo-rs/esdl/commit/722af3c52f30346f5cdb46f50bc4fcf9e48707e1)) - [@tqwewe](https://github.com/tqwewe)
- use associated error type for command trait - ([ee0849b](https://github.com/thalo-rs/esdl/commit/ee0849bbe95b7acfd5a056f77f184290d74f99b8)) - [@tqwewe](https://github.com/tqwewe)
#### Miscellaneous Chores
- add bump books to cog.toml - ([195c86c](https://github.com/thalo-rs/esdl/commit/195c86c6d64bc97232a81d92ec3997448c39c579)) - [@tqwewe](https://github.com/tqwewe)
- revert "chore: rename to esl" - ([15ad3da](https://github.com/thalo-rs/esdl/commit/15ad3da39ceac64a09d1730ba3faad8d8a42cf0f)) - [@tqwewe](https://github.com/tqwewe)
- rename to esl - ([b0a3d21](https://github.com/thalo-rs/esdl/commit/b0a3d21a70ff56737d513ffcf58150c0eaf70b43)) - [@tqwewe](https://github.com/tqwewe)
- add meta information to Cargo.toml - ([87e1ec3](https://github.com/thalo-rs/esdl/commit/87e1ec397c9b40dd1dfb3b92ea54d4464a037d9d)) - [@tqwewe](https://github.com/tqwewe)
- delete example.esdl - ([ce264de](https://github.com/thalo-rs/esdl/commit/ce264ded427384975136f29fc122aa3f19ad4ccf)) - [@tqwewe](https://github.com/tqwewe)
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).