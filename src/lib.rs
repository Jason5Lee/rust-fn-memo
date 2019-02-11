//! A library for creating the memoization of a function
//! that uses cache to improve the performance.
//! 
//! You can create the memoization with `unsync::memoize` function.
//! It uses a `HashMap` for caching.
//! 
//! ```
//! use fn_memo::{FnMemo, unsync};
//! 
//! let mul_2 = unsync::memoize(|n| {
//!     println!("Evaluating {}", n);
//!     n * 2
//! });
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
//! [the documentation of `recur-fn` crate](https://docs.rs/recur-fn/)
//! for details.
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
//! use fn_memo::{FnMemo, sync};
//! use std::thread;
//! use std::sync::Arc;
//! 
//! let mul_2 = Arc::new(sync::memoize(|n| {
//!     println!("Evaluating {}", n);
//!     n * 2
//! }));
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
pub mod unsync;
#[cfg(feature = "sync")]
pub mod sync;

/// The memoized function.
pub trait FnMemo<Arg, Output> {
    /// Calls the function. If the result of `arg` is already cached,
    /// it will return the cached result, otherwise, it caculates the result
    /// and adds it to the cache.
    fn call(&self, arg: Arg) -> Output;
    /// Clears the cache.
    fn clear_cache(&self);
}

#[cfg(test)]
mod tests {
    use std::{sync::{Arc, RwLock}, thread, time};
    use crate::{FnMemo, unsync};
    use recur_fn::*;
    #[cfg(feature = "sync")]
    use crate::sync;

    fn test_unsync(memoizer: impl Fn(&DynRecurFn<usize, usize>, &Fn(&FnMemo<usize, usize>))) {
        let cnt = std::cell::RefCell::new(0);
        memoizer(&recur_fn(|fib, n: usize| {
            *cnt.borrow_mut() += 1;
            if n <= 1 {
                n
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }), &|fib| {
            assert_eq!(5, fib.call(5));
            assert_eq!(6, *cnt.borrow());
            assert_eq!(0, fib.call(0));
            assert_eq!(1, fib.call(1));
            assert_eq!(1, fib.call(2));
            assert_eq!(2, fib.call(3));
            assert_eq!(3, fib.call(4));
            assert_eq!(5, fib.call(5));
            assert_eq!(6, *cnt.borrow());
        });
    }
    #[test]
    fn unsync_memoize_works() {
        test_unsync(|f, callback| callback(&unsync::memoize(f)))
    }

    #[test]
    fn unsync_memoize_seq_works() {
        test_unsync(|f, callback| callback(&unsync::memoize_seq(f)))
    }

    #[cfg(feature = "sync")]
    fn test_sync(memoizer: impl Fn(Box<DynRecurFn<usize, usize> + Send + Sync>,
    &Fn(Arc<FnMemo<usize, usize> + Send + Sync>))) {
        let cnt = Arc::new(RwLock::new(0));
        memoizer(Box::new({
            let cnt = Arc::clone(&cnt);
            recur_fn(move |fib, n: usize| {
                thread::sleep(time::Duration::from_millis(50));
                *cnt.write().unwrap() += 1;
                if n <= 1 {
                    n
                } else {
                    fib(n - 1) + fib(n - 2)
                }
            })
        }), &|fib| {
            assert_eq!(5, fib.call(5));
            assert_eq!(6, *cnt.read().unwrap());
            assert_eq!(1, fib.call(2));
            assert_eq!(6, *cnt.read().unwrap());

            fib.clear_cache();
            *cnt.write().unwrap() = 0;

            let mut threads = Vec::new();
            let expects = [0, 1, 1, 2, 3, 5];
            for arg in 0..=5 {
                for _ in 0..10 {
                    let test = Arc::clone(&fib);
                    threads.push(thread::spawn(move || {
                        assert_eq!(expects[arg], test.call(arg));
                    }));
                }
            }

            for thread in threads {
                thread.join().unwrap()
            }

            assert_eq!(6, *cnt.read().unwrap());
        })
    }

    #[cfg(feature = "sync")]
    #[test]
    fn sync_memoize_works() {
        test_sync(|f, callback| callback(Arc::new(sync::memoize(deref(f)))))
    }
    
    #[cfg(feature = "sync")]
    #[test]
    fn sync_memoize_rw_lock_works() {
        test_sync(|f, callback| callback(Arc::new(sync::memoize_rw_lock(deref(f)))))
    }

    #[cfg(feature = "sync")]
    #[test]
    fn sync_memoize_rw_lock_seq_works() {
        test_sync(|f, callback| callback(Arc::new(sync::memoize_rw_lock_seq(deref(f)))))
    }
}
