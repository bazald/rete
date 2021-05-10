#[allow(unused_imports)]
use super::{cnode::*, flag::*, lnode::LNode, snode::SNode, traits::*};
use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}};
use core::ptr;

pub(super) enum FindResult<'a, V> {
    NotFound,
    Found(&'a V),
}

pub(super) enum InsertResult<'a, B, V, H> {
    Found(&'a V),
    Inserted(ArcMNode<B, V, H>),
}

pub(super) enum RemoveResult<'a, B, V, H> {
    NotFound,
    Removed(ArcMNode<B, V, H>, &'a V),
}

pub(super) trait MNode <B, V: Clone, H: HasherBv<B, V>> {
    fn size(&self) -> usize;
    fn is_cnode(&self) -> bool;

    fn find(&self, value: &V, flag: Option<Flag<B>>) -> FindResult<V>;
    fn insert(&self, arc_self: ArcMNode<B, V, H>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<B, V, H>;
    fn remove(&self, value: &V, flag: Option<Flag<B>>) -> RemoveResult<B, V, H>;

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>;
}

impl <B, V: Clone, H: HasherBv<B, V>> Debug for dyn MNode<B, V, H> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        self.fmt(formatter)
    }
}

impl <B, V: Value, H: HasherBv<B, V>> Eq for dyn MNode<B, V, H> {}

impl <B, V: Value, H: HasherBv<B, V>> PartialEq for dyn MNode<B, V, H> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self as *const dyn MNode<B, V, H> as *const u8, other as *const dyn MNode<B, V, H> as *const u8)
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
