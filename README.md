# Taitank Safe

[![License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)
[![Package version](https://img.shields.io/crates/v/taitank-safe.svg)](https://crates.io/crates/taitank-safe)
[![Workflow](https://github.com/rustq/taitank-safe/actions/workflows/CI.yml/badge.svg)](https://github.com/rustq/taitank-safe/actions)

Rust binding of [tencent/taitank](https://github.com/tencent/taitank) provides safe binding to the layout library.

## Usage

```toml
[dependencies]
taitank_safe = "0.1.0"
```

```rust
use taitank_safe::*;

let mut root = node_create();

set_width(&mut root, 100.0);
set_height(&mut root, 100.0);

layout!(&mut root);
```

## Development

```shell
$ git clone git@github.com:rustq/taitank-safe.git
```

```shell
$ cd taitank-safe
```

```shell
$ cargo run --example demo
```


## License

[MIT](https://opensource.org/licenses/MIT)