# Taitank Safe

(In Progress) Rust binding of [tencent/taitank](https:://github.com/tencent/taitank) provides safe binding to the Taitank Layout Library.

## Usage

```toml
[dependencies]
taitank_safe = "0.0.1"
```

```rust
use taitank_safe::*;

let mut root = node_create();
set_width(&mut root, 100.0);
set_height(&mut root, 100.0);

let mut root_child0 = node_create();
set_flex_grow(&mut root_child0, 1.0);
set_flex_basis(&mut root_child0, 50.0);
insert_child(&mut root, &mut root_child0, 0);

let mut root_child1 = node_create();
set_flex_grow(&mut root_child1, 1.0);
insert_child(&mut root, &mut root_child1, 1);

layout!(&mut root);
```