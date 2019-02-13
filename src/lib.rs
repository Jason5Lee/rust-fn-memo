//! A library for creating the memoization of a function
//! that uses cache to improve the performance.
//!
//! You can create the memoization with `unsync::memoize` function.
//! It uses a `HashMap` for caching.
//!
//! ```
//! use fn_memo::{FnMemo, unsync, recur_fn::direct};
//!
//! let mul_2 = unsync::memoize(direct(|n| {
//!     println!("Evaluating {}", n);
//!     n * 2
//! }));
//!
//! assert_eq!(0, mul_2.call(0)); // Output "Evaluating 0."
//! assert_eq!(4, mul_2.call(2)); // Output "Evaluating 2."
//! assert_eq!(10, mul_2.call(5)); // Output "Evaluating 5."
//! assert_eq!(4, mul_2.call(2)); // No output. The result is cached.
//! mul_2.clear_cache();
//! assert_eq!(4, mul_2.call(2)); // Output "Evaluating 2."
//! ```
//!
//! The `memoize` function takes a `RecurFn` argument,
//! which allows you to memoize a recursive function and each recursion
//! result will be cached. See
//! [the API reference of `recur-fn`](https://docs.rs/recur-fn/)
//! for details.
//!
//! ```
//! use fn_memo::{FnMemo, unsync, recur_fn::recur_fn};
//!
//! let fib = unsync::memoize(recur_fn(|fib, n: usize| {
//!     println!("Evaluating {}", n);
//!     if n <= 1 {
//!         n
//!     } else {
//!         fib(n - 1) + fib(n - 2)
//!     }
//! }));
//!
//! assert_eq!(55, fib.call(10));
//! assert_eq!(5, fib.call(5));
//! ```
//!
//! The code above will output the evaluation from 0 to 10.
//! Each of them is outputed only once.
//!
//! For the sequence (i.e. the function that takes an `usize` as argument),
//! you can also use `unsync::memoize_seq`. It uses a `Vec` as a bucket
//! to cache, so it has a better performance but takes the memory
//! proportional to the largest argument of the cache.
//!
//! You can costumize the data structure of the cache by implementing
//! `unsync::Cache` trait and create memoization with `unsync::Memo::new` method.
//! For more details, see the documentation.
//!
//! The APIs under `unsync` namespace are for single-thread memoization.
//! The result of `unsync::memoize` does not `Send` even the cached function does.
//!
//! ```compile_fail
//! use std::{sync::Arc, thread};
//! use fn_memo::{FnMemo, unsync};
//!
//! let f = Arc::new(unsync::memoize(|n: i32| n));
//! thread::spawn(move || { f }); // Compile Error
//! ```
//!
//! The synchronize version APIs are under `sync` namespace.
//!
//! ```
//! use fn_memo::{FnMemo, sync::chashmap, recur_fn::direct};
//! use std::thread;
//! use std::sync::Arc;
//!
//! let mul_2 = Arc::new(chashmap::memoize(direct(|n| {
//!     println!("Evaluating {}", n);
//!     n * 2
//! })));
//!
//! let mut threads = Vec::new();
//! for _ in 0..4 {
//!     threads.push(thread::spawn({
//!         let mul_2 = Arc::clone(&mul_2);
//!         move || {
//!             for n in 0..10 {
//!                 assert_eq!(n*2, mul_2.call(n));
//!             }
//!         }
//!     }));
//! }
//! for thread in threads {
//!     thread.join().unwrap();
//! }
//! ```
//!
//! The code above will output the evaluation from 0 to 9.
//! Each of them is outputed only once.
#[cfg(feature = "sync")]
pub mod sync;
pub mod unsync;

pub use recur_fn;

/// The memoized function.
pub trait FnMemo<Arg, Output> {
    /// Calls the function. If the result of `arg` is already cached,
    /// it will return the cached result, otherwise, it caculates the result
    /// and adds it to the cache.
    fn call(&self, arg: Arg) -> Output;
    /// Clears the cache.
    fn clear_cache(&self);
}
