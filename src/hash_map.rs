use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use recur_fn::RecurFn;

/// A `FnMemo` implementation with `HashMap` and `RwLock`.
pub struct HashMapMemo<Arg, Output, F> {
    map: RwLock<HashMap<Arg, Output>>,
    f: F,
}

impl <Arg: Clone + Eq + Hash, Output: Clone,
    F: RecurFn<Arg, Output>>
    HashMapMemo<Arg, Output, F>
{
    /// Constructs a `HashMapMemo` caching function `f`.
    pub fn new(f: F) -> HashMapMemo<Arg, Output, F> {
        HashMapMemo {
            map: RwLock::new(HashMap::new()),
            f
        }
    }
}

impl <Arg: Clone + Eq + Hash, Output: Clone,
    F: RecurFn<Arg, Output>>
    crate::FnMemo<Arg> for HashMapMemo<Arg, Output, F>
{
    type Output = Output;

    fn call(&self, arg: Arg) -> Self::Output {
        if let Some(result) = self.map.read().unwrap().get(&arg) {
            return result.clone()
        }
        let result = self.f.body(|arg| self.call(arg), arg.clone());
        self.map.write().unwrap().insert(arg, result.clone());
        result
    }

    fn clear_cache(&self) {
        self.map.write().unwrap().clear();
    }
}
