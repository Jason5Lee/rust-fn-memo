mod sync;

use fn_memo::sync::chashmap;
use std::sync::Arc;
use recur_fn::from_pointer;

#[test]
fn memoize_works() {
    sync::test_sync(|f, callback| callback(Arc::new(chashmap::memoize(from_pointer(f)))))
}
