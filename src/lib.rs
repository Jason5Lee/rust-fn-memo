//! A library for creating the memoization of a function
//! that uses cache to improve the performance.

pub mod hash_map;

/// A memoization of a function that uses cache to improve
/// the performance when the same arguments are called repeated often.
pub trait FnMemo<Arg> {
    /// The return type.
    type Output;

    /// Call the function with `arg` as argument.
    /// If the result of `arg` is cached, the cache result
    /// will be returned directly.
    /// Otherwise, it will calculate the result and cache.
    /// Note that one method call may result in multiple caching,
    /// as the function may be recursive.
    fn call(&self, arg: Arg) -> Self::Output;
    /// Clear the cache.
    fn clear_cache(&self);
}

#[cfg(test)]
mod tests {
    use crate::FnMemo;
    use crate::hash_map::*;
    use recur_fn::*;

    #[test]
    fn fib_hash_map_memo_works() {
        let fib = HashMapMemo::new({
                struct Fib {}
                impl RecurFn<usize, usize> for Fib {
                    fn body<F: Fn(usize) -> usize>(&self, recur: F, arg: usize) -> usize {
                        if arg <= 1 {
                            arg
                        } else {
                            recur(arg - 1) + recur(arg - 2)
                        }
                    }
                }
                Fib {}
            });
        assert_eq!(0, fib.call(0));
        assert_eq!(1, fib.call(1));
        assert_eq!(1, fib.call(2));
        assert_eq!(2, fib.call(3));
        assert_eq!(3, fib.call(4));
        assert_eq!(5, fib.call(5));
        assert_eq!(1, fib.call(2));
    }

    #[test]
    fn fact_hash_map_memo_works() {
        let fact = HashMapMemo::new( {
                struct Fact {}
                impl RecurFn<usize, usize> for Fact {
                    fn body<F: Fn(usize) -> usize>(&self, recur: F, arg: usize) -> usize {
                        if arg == 0 { 1 } else {
                            arg * recur(arg - 1)
                        }
                    }
                }
                Fact {}
            });
        assert_eq!(1, fact.call(0));
        assert_eq!(1, fact.call(1));
        assert_eq!(2, fact.call(2));
        assert_eq!(6, fact.call(3));
        assert_eq!(24, fact.call(4));
        assert_eq!(3628800, fact.call(10));
    }

    #[test]
    fn mul2_hash_map_memo_works() {
        let mul2 = HashMapMemo::new(|n: usize| n * 2);

        assert_eq!(0, mul2.call(0));
        assert_eq!(2, mul2.call(1));
        assert_eq!(100, mul2.call(50));
        assert_eq!(2, mul2.call(1));
    }

    use std::cell::RefCell;
    #[test]
    fn hash_map_memo_cache_works() {
        let cache_cnt = RefCell::new(0usize);

        let test_f = HashMapMemo::new(|_: usize| {
            *cache_cnt.borrow_mut() += 1;
        });

        test_f.call(0); assert_eq!(1, *cache_cnt.borrow());
        test_f.call(1); assert_eq!(2, *cache_cnt.borrow());
        test_f.call(2); assert_eq!(3, *cache_cnt.borrow());
        test_f.call(0); assert_eq!(3, *cache_cnt.borrow()); // 0 is cached
        test_f.call(2); assert_eq!(3, *cache_cnt.borrow()); // 2 is cached
        test_f.call(3); assert_eq!(4, *cache_cnt.borrow());
        test_f.clear_cache();
        test_f.call(0); assert_eq!(5, *cache_cnt.borrow());
        test_f.call(1); assert_eq!(6, *cache_cnt.borrow());
    }
}
