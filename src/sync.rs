#[cfg(feature = "concurrent_hash_map")]
pub mod chashmap;
pub mod rw_lock;

use crate::FnMemo;
use once_cell::sync::OnceCell;
use recur_fn::RecurFn;
use std::sync::Arc;

/// The cache for synchronized memoization.
pub trait Cache {
    type Arg;
    type Output;

    /// Creates an empty cache.
    fn new() -> Self;
    /// Gets the cell of the `arg` in cache. Returns `None` if `arg` is not cached.
    /// This method should only acquire read lock if needed.
    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>>;
    /// Gets the cell of the `arg` in cache, creates if `arg` is not cached.
    /// This method can acquire write lock if needed.
    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>>;
    /// Clears the cache.
    fn clear(&self);
}

/// The synchronized implementation of `FnMemo`.
pub struct Memo<C, F> {
    cache: C,
    f: F,
}

impl<C: Cache, F> Memo<C, F> {
    /// Constructs a `Memo` using `C` as cache, caching function `f`.
    pub fn new(f: F) -> Self {
        Memo { cache: C::new(), f }
    }
}

impl<C: Cache, F: RecurFn<C::Arg, C::Output>> FnMemo<C::Arg, C::Output> for Memo<C, F>
where
    C::Arg: Clone,
    C::Output: Clone,
{
    fn call(&self, arg: C::Arg) -> C::Output {
        self.cache
            .get(&arg)
            .unwrap_or_else(|| self.cache.get_or_new(arg.clone()))
            .get_or_init(|| self.f.body(|arg| self.call(arg), arg))
            .clone()
    }

    fn clear_cache(&self) {
        self.cache.clear();
    }
}
