# CKANDex

[![crates.io](https://img.shields.io/crates/v/ckandex?style=flat-square)](https://crates.io/crates/ckandex)

A rust-powered [NetKAN](https://github.com/KSP-CKAN/NetKAN) resolver and indexer library, for both KSP 1 and KSP 2.

## About

This library was created for the [Wormhole](https://github.com/RedstoneWizard08/Wormhole) project, as we needed a way to query and list mods in CKAN's [NetKAN](https://github.com/KSP-CKAN/NetKAN) database. However, there wasn't an easy API to do this, as the NetKAN database is essentially a ton of JSON and YAML files crammed into a bunch of folders, so we had to develop our own solution.

## Features

- Blazingly fast (I know) query and filter system.
- Low overhead.
- Memory-efficient (somewhat).
- Well caching.
- Always up to date.
- Easy-to-use API.
- Made with Tokio and Serde.
- Easy to integrate into existing codebases.

## Example

Here's an example of CKANDex in action:

```rs
use ckandex::{refresh_data, run_server, KSP};
use dotenv::dotenv;
use tokio::main;

#[main]
pub async fn main() {
    dotenv().ok();

    refresh_data(KSP::KSP2, "netkan-ksp2").await;
    run_server("netkan-ksp2".to_string()).await;
}
```

For more examples, check out the [example/](example/) folder in this repo.

## Contributing

This library is still WIP! Please don't be afraid to make feature requests, pull requests, and anything else. If you have any questions, or you have found any bugs, please create an issue! I'll be happy to help as soon as I can.

## Code Style

The code style is as follows:

- 4-space tab (spaces, not tabs)
- Brackets on same lines as declarations (`pub async fn main() {` not `pub async fn main()\n{`)
- Declared return (`return ...;` not `...`) [Shut up, clippy. I prefer this.]
- Look at the rest of the codebase for more.
