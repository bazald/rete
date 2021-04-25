#[allow(unused_imports)]
use super::{cnode::*, lnode::LNode, snode::SNode};
use alloc::fmt::{self, Debug, Formatter};
use core::hash::Hash;

pub(super) trait MNode <T: Clone + Eq + PartialEq + Hash> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>;
}

impl <T: Clone + Eq + PartialEq + Hash> Debug for dyn MNode<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        self.fmt(formatter)
    }
}
