# fluffyf

fluffyf is pretty much a rust library and application implementing API requests
to E621/E926 with the primary focus of downloading posts and pools.

Before anything, please read [E926's API documentation](https://e926.net/wiki_pages/2425#api),
it helps, trust me.

If you're new to asynchronous programming in Rust (like me), you should read
[the async book](https://rust-lang.github.io/async-book/), and maybe
even [tokio's tutorial](https://tokio.rs/tokio/tutorial).

## Current State

`fluffyf` is currently
- barely usable
- missing a lot of features
- still lackluster in proper async implementation
- severely underdocumented

`fluffyget` is currently
- lacking pool fetching
- can't handle file based input
- etc

`fluffypost` is currently **unusable and not ready yet**

## Installation

The entire fluffyf project consists of two parts: library and binary.

### Library

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fluffyf = "0.1"
```

### Binary

Run the following on your favorite shell:

```sh
$ cargo install fluffyf
```

and assuming that you have added `$HOME/.cargo/bin` to PATH:

```sh
$ fluffyget --help
```

should output a help message.

## Features

Features can be enabled through `Cargo.toml`, either by:

```toml
[dependencies]
fluffyf = { version = "0.1", features = ["rate-limit"] }
```

or:

```toml
[dependencies]

[dependencies.fluffyf]
default-features = false
features = ["rate-limit"]
version = "0.1"
```

## Versioning

Regarding versioning and breaking changes.

### SemVer

This project uses [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html). However, as of January 2024, this project has not reached
version `1.0.0` yet.<br>
As such, **expect extreme amounts of breaking changes per minor version changes**

### MSRV

Due to the nature of async programming in Rust, fluffyf is almost forced
to use a very high (or "cutting-edge") minimum supported Rust version.

Note: after `1.0.0`, MSRV changes will only happen every major version change.

### Changelogging

This project follows [keep a changelog version 1.1.0](https://keepachangelog.com/en/1.1.0/).

## License
This project is under a dual license of:

- BSD 2-Clause License
- Apache License, Version 2.0

at Your (as per Apache 2.0 License) option.

Unless explicitly stated otherwise, any Contribution made by any Contributor to the Work
shall be implicitly dual licensed as stated above.

### Some Attribution

@nasso, @playfulkittykat for `rate_limit` in [rs621](https://github.com/nasso/rs621)

## This Project is Under Construction 🚧

Create an issue, pull request (if you know how to implement the feature) or contact me directly if you have a suggestion.
