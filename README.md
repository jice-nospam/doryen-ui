# doryen-ui

[![Build Status](https://travis-ci.org/jice-nospam/doryen-ui.svg)](https://travis-ci.org/jice-nospam/doryen-ui)
[![Documentation](https://docs.rs/doryen-ui/badge.svg)](https://docs.rs/doryen-ui)
[![crates.io](https://meritbadge.herokuapp.com/doryen-ui)](https://crates.io/crates/doryen-ui)
[![License: MIT](https://img.shields.io/badge/license-MIT-informational.svg)](#license)

A pure rust immediate user interface library for [doryen-rs](https://github.com/jice-nospam/doryen-rs).

# compilation instructions
* install rust : https://www.rust-lang.org/learn/get-started

## native compilation
```
cargo run --example showcase --features=doryen
```

## web assembly compilation
```
rustup target install wasm32-unknown-unknown
cargo install cargo-web
cargo web start --example showcase --features=doryen
```

# usage
Cargo.toml :
```toml
[dependency]
doryen-ui={ version = "*", features=["doryen"] }
```

main.rs :
```rust

fn main() {
}
```

# license

This code is released under the MIT license.

# contributions

You can contribute to this library through pull requests. If you do so, please update the CHANGELOG.md and CREDITS.md files. If you provide a new feature, consider adding an example as a tutorial/showcase.
