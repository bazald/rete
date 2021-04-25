#[allow(unused_imports)]
use super::{cnode::*, lnode::LNode, snode::SNode};
use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::{hash::Hash, ptr};

pub(super) enum FindResult<'a, T> {
    NotFound,
    Found(&'a T),
}

pub(super) enum InsertResult<'a, T> {
    Found(&'a T),
    Inserted(Arc<dyn MNode<T>>, &'a T),
}

pub(super) enum RemoveResult<'a, T> {
    NotFound,
    Found(Arc<dyn MNode<T>>, &'a T),
    FoundInSNode(&'a T),
}

pub(super) trait MNode <T: Clone + Debug + Eq + PartialEq + Hash + 'static> {
    fn find<'a>(&'a self, value: &T) -> FindResult<T>;
    fn insert<'a>(&'a self, value: Cow<T>) -> InsertResult<T>;
    fn remove<'a>(&'a self, value: &T) -> RemoveResult<T>;

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>;
}

impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> Debug for dyn MNode<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        self.fmt(formatter)
    }
}

impl <T: Clone + Eq + PartialEq + Hash + 'static> Eq for dyn MNode<T> {}

impl <T: Clone + Eq + PartialEq + Hash + 'static> PartialEq<dyn MNode<T>> for dyn MNode<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self as *const dyn MNode<T> as *const u8, other as *const dyn MNode<T> as *const u8)
    }
}
