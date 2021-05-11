use crate::bit_indexed_array::*;
use super::{flag::*, lnode, mnode::*, snode::{self, *}, traits::*};
use alloc::{boxed::Box, borrow::Cow, fmt::Debug, sync::Arc};

#[derive(Debug)]
pub(super) struct CNode <B: Bits, V: Value, H: 'static> {
    nodes: Arc<dyn BitIndexedArray::<B, MNode<B, V, H>, usize>>,
}

impl<B: Bits, V: Value, H: HasherBv<B, V>> CNode<B, V, H> {
    pub(super) fn new(nodes: Box<dyn BitIndexedArray::<B, MNode<B, V, H>, usize> + 'static>) -> Self {
        Self { nodes: nodes.into() }
    }
}

impl <B: Bits, V: Value, H: HasherBv<B, V>> CNode<B, V, H> {
    pub(super) fn size(&self) -> usize {
        *self.nodes.extra()
    }

    pub(super) fn find<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> FindResult<'a, V> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node {
                MNode::C(cnode) => cnode.find(value, match flag {Some(flag) => flag.next(), None => None}),
                MNode::L(lnode) => lnode.find(value),
                MNode::S(snode) => {
                    if *snode.get() == *value {
                        FindResult::Found(snode.get())
                    }
                    else {
                        FindResult::NotFound
                    }
                },
            },
            Err(_) => FindResult::NotFound
        }
    }

    pub(super) fn insert<'a>(&'a self, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node {
                MNode::C(cnode) => match cnode.insert(value, flag.as_ref().unwrap().next()) {
                    InsertResult::Found(reference) => InsertResult::Found(reference),
                    InsertResult::InsertedC(cnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::C(cnode)), Cow::Owned(self.size() + 1)).unwrap())),
                    InsertResult::InsertedL(lnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::L(lnode)), Cow::Owned(self.size() + 1)).unwrap())),
                },
                MNode::L(lnode) => match lnode::insert(&lnode, value, flag.as_ref().unwrap().next()) {
                    InsertResult::Found(reference) => InsertResult::Found(reference),
                    InsertResult::InsertedC(cnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::C(cnode)), Cow::Owned(self.size() + 1)).unwrap())),
                    InsertResult::InsertedL(lnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::L(lnode)), Cow::Owned(self.size() + 1)).unwrap())),
                },
                MNode::S(snode) => match snode::insert(&snode, value, flag.as_ref().unwrap().next()) {
                    InsertResult::Found(reference) => InsertResult::Found(reference),
                    InsertResult::InsertedC(cnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::C(cnode)), Cow::Owned(self.size() + 1)).unwrap())),
                    InsertResult::InsertedL(lnode) => InsertResult::InsertedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::L(lnode)), Cow::Owned(self.size() + 1)).unwrap())),
                },
            },
            Err(_) => InsertResult::InsertedC(Self::new(self.nodes.inserted(flag.unwrap().flag, Cow::Owned(MNode::S(SNode::new(value.into_owned()))), Cow::Owned(self.size() + 1)).unwrap()))
        }
    }

    pub(super) fn remove<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> RemoveResult<'a, B, V, H> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node.remove(value, flag.as_ref().unwrap().next()) {
                RemoveResult::NotFound => RemoveResult::NotFound,
                RemoveResult::RemovedC(node, reference) => {
                    if node.size() == 0 {
                        panic!()
                    }
                    RemoveResult::RemovedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::C(node)), Cow::Owned(self.size() - 1)).unwrap()), reference)
                },
                RemoveResult::RemovedL(node, reference) => {
                    RemoveResult::RemovedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::L(node)), Cow::Owned(self.size() - 1)).unwrap()), reference)
                },
                RemoveResult::RemovedS(node, reference) => {
                    RemoveResult::RemovedC(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(MNode::S(node)), Cow::Owned(self.size() - 1)).unwrap()), reference)
                },
                RemoveResult::RemovedZ(reference) => {
                    RemoveResult::RemovedC(Self::new(self.nodes.removed(flag.unwrap().flag, Cow::Owned(self.size() - 1)).unwrap()), reference)
                },
            },
            Err(_) => RemoveResult::NotFound
        }
    }
}

impl<B: Bits, V: Value, H: 'static> Clone for CNode<B, V, H> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone()
        }
    }
}

impl<B: Bits, V: Value, H: HasherBv<B, V>> Default for CNode<B, V, H> {
    fn default() -> Self {
        CNode::<B, V, H>::new(default_bit_indexed_array())
    }
}
