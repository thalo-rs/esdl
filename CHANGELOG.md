# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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