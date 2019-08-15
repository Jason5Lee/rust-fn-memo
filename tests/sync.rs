use fn_memo::FnMemo;
use recur_fn::*;
use std::{
    sync::{Arc, RwLock},
    thread, time,
};

pub fn test_sync(
    memoizer: impl Fn(
        Box<dyn DynRecurFn<usize, usize> + Send + Sync>,
        &dyn Fn(Arc<dyn FnMemo<usize, usize> + Send + Sync>),
    ),
) {
    let cnt = Arc::new(RwLock::new(0));
    memoizer(
        Box::new({
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
        }),
        &|fib| {
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
        },
    )
}
