#[allow(unused_imports)]
use super::{cnode::*, flag::*, lnode::LNode, snode::SNode, traits::*};
use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::ptr;

pub(super) enum FindResult<'a, V> {
    NotFound,
    Found(&'a V),
}

pub(super) enum InsertResult<'a, B, V> {
    Found(&'a V),
    Inserted(Arc<dyn MNode<B, V>>),
}

pub(super) enum RemoveResult<'a, B, V> {
    NotFound,
    Removed(Arc<dyn MNode<B, V>>, &'a V),
}

pub(super) trait MNode <B, V: Clone> {
    fn size(&self) -> usize;
    fn is_cnode(&self) -> bool;

    fn find<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> FindResult<V>;
    fn insert<'a>(&'a self, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<B, V>;
    fn remove<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> RemoveResult<B, V>;

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>;
}

impl <B, V: Clone> Debug for dyn MNode<B, V> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        self.fmt(formatter)
    }
}

impl <B, V: Value> Eq for dyn MNode<B, V> {}

impl <B, V: Value> PartialEq for dyn MNode<B, V> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self as *const dyn MNode<B, V> as *const u8, other as *const dyn MNode<B, V> as *const u8)
    }
}

#[allow(unused_macros)]
macro_rules! assert_found_eq {
    ( $found:expr, $expected:expr ) => {
        match $found {
            FindResult::Found(reference) => assert_eq!(*reference, $expected),
            FindResult::NotFound => panic!()
        }
    };
}

#[allow(unused_macros)]
macro_rules! assert_found_none {
    ( $found:expr ) => {
        match $found {
            FindResult::Found(_reference) => panic!(),
            FindResult::NotFound => {}
        }
    };
}
