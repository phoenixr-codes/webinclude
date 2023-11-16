# webinclude

## Example

```rust
use webinclude::webinclude;
webinclude!("https://raw.githubusercontent.com/phoenixr-codes/webinclude/master/say.rs");

fn main() {
    say("Hi");
}
```


## Quickstart

```console
git clone https://github.com/phoenixr-codes/webinclude.git
cd webinclude
```

```console
cargo run --example example
```

```console
cargo doc --no-deps --open
```


## Why???

> <https://youtu.be/4vSyqK3SK-0?feature=shared&t=2755>

This is obviously just made out of fun to prove that you can indeed `include` files over HTTP in Rust.
