# FnMemo

A Rust library for creating the memoization of a function.

Documentation: [API reference](https://docs.rs/fn-memo)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fn-memo = "0.2"
```

By default `fn-memo` includes synchronized APIs, which introduces some extra dependencies. To disable synchronization to reduce the depdendencies, use following configuration.

```toml
[dependencies]
fn-memo = { version = "0.2", default-features = false }
```
