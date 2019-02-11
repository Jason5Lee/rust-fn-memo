use crate::FnMemo;
use recur_fn::RecurFn;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;

/// The cache for single-thread memoization.
pub trait Cache {
    type Arg;
    type Output;

    /// Create an empty cache.
    fn new() -> Self;
    /// Gets the cached result of `arg`. If it is not cached,
    /// returns `None`.
    fn get(&self, arg: &Self::Arg) -> Option<&Self::Output>;
    /// Caches the `arg` with `result`.
    fn cache(&mut self, arg: Self::Arg, result: Self::Output);
    /// Clears the cache.
    fn clear(&mut self);
}

/// Use `HashMap` as `Cache`.
impl<Arg: Clone + Eq + Hash, Output: Clone> Cache for HashMap<Arg, Output> {
    type Arg = Arg;
    type Output = Output;

    fn new() -> Self {
        HashMap::new()
    }
    fn get(&self, arg: &Arg) -> Option<&Output> {
        HashMap::get(self, arg);
        self.get(arg)
    }

    fn cache(&mut self, arg: Arg, result: Output) {
        self.insert(arg, result);
    }

    fn clear(&mut self) {
        self.clear();
    }
}

/// Use `Vec` as `Cache` for sequences.
impl<Output: Clone> Cache for Vec<Option<Output>> {
    type Arg = usize;
    type Output = Output;

    fn new() -> Self {
        Vec::new()
    }

    fn get(&self, arg: &usize) -> Option<&Output> {
        self.as_slice().get(*arg)?.as_ref()
    }

    fn cache(&mut self, arg: usize, result: Output) {
        if arg >= self.len() {
            self.resize(arg + 1, None);
        }
        self[arg] = Some(result);
    }

    fn clear(&mut self) {
        self.clear();
    }
}

/// The single-thread implementation of `FnMemo`.
pub struct Memo<C, F> {
    cache: UnsafeCell<C>,
    f: F,
}

impl<C: Cache, F: RecurFn<C::Arg, C::Output>> Memo<C, F>
where
    C::Arg: Clone,
    C::Output: Clone,
{
    /// Constructs a `Memo` using `C` as cache, caching function `f`.
    pub fn new(f: F) -> Memo<C, F> {
        Memo {
            cache: UnsafeCell::new(C::new()),
            f,
        }
    }
}

impl<C: Cache, F: RecurFn<C::Arg, C::Output>> FnMemo<C::Arg, C::Output> for Memo<C, F>
where
    C::Arg: Clone,
    C::Output: Clone,
{
    fn call(&self, arg: C::Arg) -> C::Output {
        if let Some(result) = unsafe { &*self.cache.get() }.get(&arg) {
            return result.clone();
        }

        let result = self.f.body(|arg| self.call(arg), arg.clone());
        unsafe { &mut *self.cache.get() }.cache(arg, result.clone());
        result
    }

    fn clear_cache(&self) {
        unsafe { &mut *self.cache.get() }.clear()
    }
}

/// Creates a memoization of `f` using `HashMap` as cache.
pub fn memoize<Arg, Output, F>(f: F) -> impl FnMemo<Arg, Output>
where
    Arg: Clone + Eq + Hash,
    Output: Clone,
    F: RecurFn<Arg, Output>,
{
    Memo::<std::collections::HashMap<_, _>, _>::new(f)
}

/// Creates a memoization of the sequence `f` using `Vec` as cache.
pub fn memoize_seq<Output, F>(f: F) -> impl FnMemo<usize, Output>
where
    Output: Clone,
    F: RecurFn<usize, Output>,
{
    Memo::<Vec<_>, _>::new(f)
}
