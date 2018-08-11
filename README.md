# Avocado, the strongly-typed MongoDB driver

[![Avocado on crates.io](https://img.shields.io/crates/v/avocado.svg)](https://crates.io/crates/avocado)
[![Avocado on docs.rs](https://docs.rs/avocado/badge.svg)](https://docs.rs/avocado)
[![Avocado Download](https://img.shields.io/crates/d/avocado.svg)](https://crates.io/crates/avocado)
[![Avocado License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/H2CO3/avocado/blob/master/LICENSE.txt)
[![Lines of Code](https://tokei.rs/b1/github/H2CO3/avocado)](https://github.com/Aaronepower/tokei)
[![Twitter](https://img.shields.io/badge/twitter-@H2CO3_iOS-blue.svg?style=flat&colorB=64A5DE&label=Twitter)](http://twitter.com/H2CO3_iOS)

## Usage

* `cargo doc --no-deps --open`
* Check out the [`examples/`](https://github.com/H2CO3/avocado/blob/master/examples/) folder
* More docs are coming!

## TODO:

* Wait for `bson-rs` crate to fix newtype-struct-serialization-as-array bug
* Write documentation in `lib.rs` doc comments
* Write module-level tests that only check if domain model objects serialize correctly etc.
* Write integration tests that exercise the library using an actual, running MongoDB database
* Add a `prelude` so users don't have to explicitly import stuff from `filter`,
  `update`, etc. in order to use the `filter!{}` macro and similar ones
* Auto-derive `Doc` trait; respect Serde renaming when obtaining type name!
* Auto-derive `dsl::ops` traits (`Query`, `Update`, `Upsert`, `Delete`, `Aggregate`, etc.)
