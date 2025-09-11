# OpenRTB 2.6 types for Rust

This is a set of type definitions derived from the OpenRTB 2.6 specification for Rust.

**DISCLAIMER**: This was generated using an LLM, and accuracy is not guaranteed. If you notice any bugs please submit a PR.

**NOTE**: I do not claim any of the types provided in this crate as my own. They are a simply transformation of the specification provided by IAB. Any usage of this crate **MUST** comply with the license terms of the OpenRTB specification.

## Usage

For the time being this crate will not be published to crates.io, but it can be freely used by adding the following to your `Cargo.toml`:

```toml
openrtb26 = { git = "https://github.com/Minebomber/openrtb26-rs" }
```

Types are exported by the `openrtb26` module.

```rust
use openrtb26::BidRequest;

fn main() {

}
```
