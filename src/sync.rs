use once_cell::sync::OnceCell;
use chashmap::CHashMap;
use std::sync::{Arc, RwLock};
use std::hash::Hash;
use crate::FnMemo;
use recur_fn::RecurFn;

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

/// Use `CHashMap` as `Cache`.
impl <Arg, Output> Cache for CHashMap<Arg, Arc<OnceCell<Output>>> 
where Arg: PartialEq + Hash {
    type Arg = Arg;
    type Output = Output;

    fn new() -> Self {
        CHashMap::new()
    }

    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>> {
        self.get(arg).map(|arc| Arc::clone(&arc))
    }

    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>> {
        let cell = std::cell::UnsafeCell::new(
            unsafe { std::mem::uninitialized() }
        );
        
        self.upsert(arg, || {
            let arc = Arc::new(OnceCell::INIT);
            unsafe { std::ptr::write(cell.get(), Arc::clone(&arc)) };
            arc
        }, |arc| {
            unsafe { std::ptr::write(cell.get(), Arc::clone(arc)) };
        });

        cell.into_inner()
    }

    fn clear(&self) {
        self.clear();
    }
}

use std::collections::HashMap;
/// Use `HashMap` with `RwLock` as `Cache`.
impl <Arg, Output> Cache for RwLock<HashMap<Arg, Arc<OnceCell<Output>>>>
where Arg: Eq + Hash {
    type Arg = Arg;
    type Output = Output;

    fn new() -> Self {
        RwLock::new(HashMap::new())
    }

    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>> {
       self.read().unwrap().get(arg).map(|arc| Arc::clone(arc))
    }

    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>> {
        Arc::clone(self.write().unwrap().entry(arg)
            .or_insert_with(|| Arc::new(OnceCell::INIT)))
    }

    fn clear(&self) {
        self.write().unwrap().clear()
    }
}

/// Use `Vec` with `RwLock` as `Cache` for sequences.
impl <Output> Cache for RwLock<Vec<Arc<OnceCell<Output>>>> {
    type Arg = usize;
    type Output = Output;

    fn new() -> Self {
        RwLock::new(Vec::new())
    }

    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>> {
       self.read().unwrap().get(*arg).map(|arc| Arc::clone(arc))
    }

    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>> {
        let mut write = self.write().unwrap();

        if arg >= write.len() {
            let delta: usize = arg+1 - write.len();
            write.reserve(delta);
            while write.len() <= arg {
                write.push(Arc::new(OnceCell::INIT));
            }
        }
        Arc::clone(&write[arg])
    }

    fn clear(&self) {
        self.write().unwrap().clear()
    }
}

/// The synchronized implementation of `FnMemo`.
pub struct Memo<C, F> {
    cache: C,
    f: F,
}

impl <C: Cache, F> Memo<C, F> {
    /// Constructs a `Memo` using `C` as cache, caching function `f`.
    pub fn new(f: F) -> Self {
        Memo {
            cache: C::new(),
            f
        }
    }
}

impl <C: Cache, F: RecurFn<C::Arg, C::Output>> FnMemo<C::Arg, C::Output> 
for Memo<C, F> where C::Arg: Clone, C::Output: Clone {
    fn call(&self, arg: C::Arg) -> C::Output {
        self.cache.get(&arg)
            .unwrap_or_else(|| self.cache.get_or_new(arg.clone()))
            .get_or_init(|| {
                self.f.body(|arg| self.call(arg), arg)
            }).clone()
    }

    fn clear_cache(&self) {
        self.cache.clear();
    }
}

/// Creates a synchronized memoization of `f` using `CHashMap` as cache.
pub fn memoize<Arg, Output, F>(f: F) -> impl FnMemo<Arg, Output> 
where Arg: Clone + PartialEq + Hash, Output: Clone, F: RecurFn<Arg, Output> {
    Memo::<CHashMap<_, _>, _>::new(f)
}

/// Creates a synchronized memoization of `f` using `HashMap` with `RwLock` as cache.
pub fn memoize_rw_lock<Arg, Output, F>(f: F) -> impl FnMemo<Arg, Output>  
where Arg: Clone + Eq + Hash, Output: Clone, F: RecurFn<Arg, Output> {
    Memo::<RwLock<HashMap<_, _>>, _>::new(f)
}

/// Creates a synchronized memoization of the sequence `f` 
/// using `Vec` with `RwLock` as cache.
pub fn memoize_rw_lock_seq<Output, F>(f: F) -> impl FnMemo<usize, Output>
where Output: Clone, F: RecurFn<usize, Output> {
    Memo::<RwLock<Vec<_>>, _>::new(f)
}