# FnMemo

[![Build Status](https://jason5lee.visualstudio.com/rust-pipelines/_apis/build/status/rust-fn-memo-CI?branchName=master)](https://jason5lee.visualstudio.com/rust-pipelines/_build/latest?definitionId=2&branchName=master)

A Rust library for creating the memoization of a function.

Documentation: [API reference](https://docs.rs/fn-memo)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fn-memo = "1.0"
```

By default `fn-memo` includes synchronized APIs, which introduces some extra dependencies. To disable synchronization to reduce the depdendencies, use following configuration.

```toml
[dependencies]
fn-memo = { version = "1.0", default-features = false }
```
