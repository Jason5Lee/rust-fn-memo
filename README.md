# FnMemo

[![Build Status](https://dev.azure.com/jason5lee/rust-fn-memo/_apis/build/status/Jason5Lee.rust-fn-memo?branchName=master)](https://dev.azure.com/jason5lee/rust-fn-memo/_build/latest?definitionId=13&branchName=master)

A Rust library for creating the memoization of a function.

Documentation: [API reference](https://docs.rs/fn-memo)

## Usage

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
fn-memo = "1.2"
```

By default `fn-memo` includes synchronized APIs, which introduces related dependencies.
If you only use the memoization in one thread and want to reduce dependencies, use following configuration.

```toml
[dependencies]
fn-memo = { version = "1.2", default-features = false }
```
