use chashmap::CHashMap;
use once_cell::sync::OnceCell;
use recur_fn::RecurFn;
use std::hash::Hash;
use std::sync::Arc;

/// Use `CHashMap` as `Cache`.
impl<Arg, Output> super::Cache for CHashMap<Arg, Arc<OnceCell<Output>>>
where
    Arg: Eq + Hash,
{
    type Arg = Arg;
    type Output = Output;

    fn new() -> Self {
        CHashMap::new()
    }

    fn get(&self, arg: &Self::Arg) -> Option<Arc<OnceCell<Self::Output>>> {
        self.get(arg).map(|arc| Arc::clone(&arc))
    }

    fn get_or_new(&self, arg: Self::Arg) -> Arc<OnceCell<Self::Output>> {
        let cell = std::cell::UnsafeCell::new(unsafe { std::mem::uninitialized() });

        self.upsert(
            arg,
            || {
                let arc = Arc::new(OnceCell::INIT);
                unsafe { std::ptr::write(cell.get(), Arc::clone(&arc)) };
                arc
            },
            |arc| {
                unsafe { std::ptr::write(cell.get(), Arc::clone(arc)) };
            },
        );

        cell.into_inner()
    }

    fn clear(&self) {
        self.clear();
    }
}

/// Creates a synchronized memoization of `f` using `CHashMap` as cache.
pub fn memoize<Arg, Output, F>(f: F) -> impl crate::FnMemo<Arg, Output>
where
    Arg: Clone + Eq + Hash,
    Output: Clone,
    F: RecurFn<Arg, Output>,
{
    super::Memo::<chashmap::CHashMap<_, _>, _>::new(f)
}
