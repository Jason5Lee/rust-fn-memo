use fn_memo::{unsync, FnMemo};
use recur_fn::*;

fn test_unsync(memoizer: impl Fn(&dyn DynRecurFn<usize, usize>, &dyn Fn(&dyn FnMemo<usize, usize>))) {
    let cnt = std::cell::RefCell::new(0);
    memoizer(
        &recur_fn(|fib, n: usize| {
            *cnt.borrow_mut() += 1;
            if n <= 1 {
                n
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }),
        &|fib| {
            assert_eq!(5, fib.call(5));
            assert_eq!(6, *cnt.borrow());
            assert_eq!(0, fib.call(0));
            assert_eq!(1, fib.call(1));
            assert_eq!(1, fib.call(2));
            assert_eq!(2, fib.call(3));
            assert_eq!(3, fib.call(4));
            assert_eq!(5, fib.call(5));
            assert_eq!(6, *cnt.borrow());
        },
    );
}

#[test]
fn memoize_works() {
    test_unsync(|f, callback| callback(&unsync::memoize(from_pointer(f))))
}

#[test]
fn emoize_seq_works() {
    test_unsync(|f, callback| callback(&unsync::memoize_seq(from_pointer(f))))
}
