use super::ttl_wrap::TtlWrap;
use super::Store;

pub trait StoreExt<K, V>: Store<K, V> + Sized {
    fn into_ttl(self) -> TtlWrap<Self, K, V> {
        TtlWrap::new(self)
    }
}

impl<S, K, V> StoreExt<K, V> for S where S: Store<K, V> {}
