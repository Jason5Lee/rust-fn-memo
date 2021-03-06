mod sync;

use fn_memo::sync::rw_lock;
use recur_fn::from_pointer;
use std::sync::Arc;

#[test]
fn memoize_works() {
    sync::test_sync(|f, callback| callback(Arc::new(rw_lock::memoize(from_pointer(f)))))
}

#[test]
fn memoize_seq_works() {
    sync::test_sync(|f, callback| callback(Arc::new(rw_lock::memoize_seq(from_pointer(f)))))
}
