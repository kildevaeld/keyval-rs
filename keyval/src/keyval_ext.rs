use super::ttl_wrap::TtlWrap;
use super::Store;

pub trait StoreExt: Store + Sized {
    fn into_ttl(self) -> TtlWrap<Self> {
        TtlWrap::new(self)
    }
}

impl<S> StoreExt for S where S: Store {}
