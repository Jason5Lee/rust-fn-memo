use chashmap::CHashMap;
use once_cell::sync::OnceCell;
use recur_fn::RecurFn;
use std::cell::UnsafeCell;
use std::hash::Hash;
use std::mem::MaybeUninit;
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
        // `cell` stores a clone of the `Arc` in cache.
        // `UnsafeCell<MaybeUninit<_>>` is used here because I assume
        // the `upsert` call below will execute exactly one of its
        // `insert` or `update` parameter, which means, `cell` will be written, and only
        // be written once. So it's safe.
        let cell: UnsafeCell<MaybeUninit<Arc<OnceCell<Output>>>> =
            UnsafeCell::new(MaybeUninit::uninit());

        self.upsert(
            arg,
            || {
                let arc = Arc::new(OnceCell::new());
                unsafe { (*cell.get()).as_mut_ptr().write(Arc::clone(&arc)) };
                arc
            },
            |arc| {
                unsafe { (*cell.get()).as_mut_ptr().write(Arc::clone(arc)) };
            },
        );

        unsafe { cell.into_inner().assume_init() }
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
