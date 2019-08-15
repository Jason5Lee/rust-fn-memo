use once_cell::sync::OnceCell;
use recur_fn::RecurFn;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

/// Use `HashMap` with `RwLock` as `Cache`.
impl<Arg, Output> super::Cache for RwLock<HashMap<Arg, Arc<OnceCell<Output>>>>
where
    Arg: Eq + Hash,
{
    type Arg = Arg;
    type Output = Output;

    fn new() -> Self {
        RwLock::new(HashMap::new())
    }

    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>> {
        self.read().unwrap().get(arg).map(|arc| Arc::clone(arc))
    }

    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>> {
        Arc::clone(
            self.write()
                .unwrap()
                .entry(arg)
                .or_insert_with(|| Arc::new(OnceCell::new())),
        )
    }

    fn clear(&self) {
        self.write().unwrap().clear()
    }
}

/// Creates a synchronized memoization of `f` using `HashMap` with `RwLock` as cache.
pub fn memoize<Arg, Output, F>(f: F) -> impl crate::FnMemo<Arg, Output>
where
    Arg: Clone + Eq + Hash,
    Output: Clone,
    F: RecurFn<Arg, Output>,
{
    super::Memo::<RwLock<HashMap<_, _>>, _>::new(f)
}

/// Use `Vec` with `RwLock` as `Cache` for sequences.
impl<Output> super::Cache for RwLock<Vec<Arc<OnceCell<Output>>>> {
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
            let delta: usize = arg + 1 - write.len();
            write.reserve(delta);
            while write.len() <= arg {
                write.push(Arc::new(OnceCell::new()));
            }
        }
        Arc::clone(&write[arg])
    }

    fn clear(&self) {
        self.write().unwrap().clear()
    }
}

/// Creates a synchronized memoization of the sequence `f`
/// using `Vec` with `RwLock` as cache.
pub fn memoize_seq<Output, F>(f: F) -> impl crate::FnMemo<usize, Output>
where
    Output: Clone,
    F: RecurFn<usize, Output>,
{
    super::Memo::<RwLock<Vec<_>>, _>::new(f)
}
