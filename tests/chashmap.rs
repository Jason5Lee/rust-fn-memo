mod sync;

use fn_memo::sync::chashmap;
use recur_fn::from_pointer;
use std::sync::Arc;

#[test]
fn memoize_works() {
    sync::test_sync(|f, callback| callback(Arc::new(chashmap::memoize(from_pointer(f)))))
}
