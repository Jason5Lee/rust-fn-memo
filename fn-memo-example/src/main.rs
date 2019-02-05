use fn_memo::{FnMemo, unsync, sync};
use std::thread;
use std::sync::Arc;

fn unsync_example() {
    let mul_2 = unsync::memoize(|n| {
        println!("Evaluating {}", n);
        n * 2
    });
    
    assert_eq!(0, mul_2.call(0)); // Output "Evaluating 0."
    assert_eq!(4, mul_2.call(2)); // Output "Evaluating 2."
    assert_eq!(10, mul_2.call(5)); // Output "Evaluating 5."
    assert_eq!(4, mul_2.call(2)); // No output. The result is cached.
    mul_2.clear_cache();
    assert_eq!(4, mul_2.call(2)); // Output "Evaluating 2."
}

fn sync_example() {
    let mul_2 = Arc::new(sync::memoize(|n| {
        println!("Evaluating {}", n);
        n * 2
    }));
    
    let mut threads = Vec::new();
    for _ in 0..4 {
        threads.push(thread::spawn({
            let mul_2 = Arc::clone(&mul_2);
            move || {
                for n in 0..10 {
                    assert_eq!(n*2, mul_2.call(n));
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