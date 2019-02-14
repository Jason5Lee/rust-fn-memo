use fn_memo::{
    recur_fn::{direct, recur_fn},
    sync::chashmap,
    unsync, FnMemo,
};
use std::sync::Arc;
use std::thread;

fn unsync_example() {
    let fib = unsync::memoize(recur_fn(|fib, n: usize| {
        println!("Evaluating {}", n);
        if n <= 1 {
            n
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }));

    assert_eq!(55, fib.call(10));
    assert_eq!(5, fib.call(5));
}

fn sync_example() {
    let mul_2 = Arc::new(chashmap::memoize(direct(|n| {
        println!("Evaluating {}", n);
        n * 2
    })));

    let mut threads = Vec::new();
    for _ in 0..4 {
        threads.push(thread::spawn({
            let mul_2 = Arc::clone(&mul_2);
            move || {
                for n in 0..10 {
                    assert_eq!(n * 2, mul_2.call(n));
                }
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
fn main() {
    println!("============ unsync ============");
    unsync_example();
    println!("\n============ sync ============");
    sync_example();
}
