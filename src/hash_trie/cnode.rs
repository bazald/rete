use crate::bit_indexed_array::*;
use super::{flag::*, mnode::*, snode::*, traits::*};
use alloc::{boxed::Box, borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::ops::*;

pub(super) type ArcMNode<B, V, H> = Arc<dyn MNode<B, V, H>>;

#[derive(Clone, Debug)]
pub(super) struct CNode <B, V: Value, H: HasherBv<B, V>> {
    nodes: Box<dyn BitIndexedArray::<B, ArcMNode<B, V, H>, usize>>,
}

impl<B: AsUsize + BitAnd + CountOnes + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + Shr<usize>, V: Value, H: HasherBv<B, V>> CNode<B, V, H> {
    pub(super) fn new(nodes: Box<dyn BitIndexedArray::<B, ArcMNode<B, V, H>, usize>>) -> Self {
        Self { nodes }
    }
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> MNode<B, V, H> for CNode<B, V, H> {
    fn size(&self) -> usize {
        *self.nodes.extra()
    }

    fn is_cnode(&self) -> bool {
        true
    }

    fn find<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> FindResult<'a, V> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => node.find(value, flag.unwrap().next()),
            Err(_) => FindResult::NotFound
        }
    }

    fn insert<'a>(&'a self, _arc_self: ArcMNode<B, V, H>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node.insert(node.clone(), value, flag.as_ref().unwrap().next()) {
                InsertResult::Found(reference) => InsertResult::Found(reference),
                InsertResult::Inserted(node) => InsertResult::Inserted(Arc::new(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(node), Cow::Owned(self.size() + 1)).unwrap())))
            },
            Err(_) => {
                let node: Arc::<dyn MNode::<B, V, H>> = Arc::new(SNode::<V>::new(value.into_owned()));
                InsertResult::Inserted(Arc::new(Self::new(self.nodes.inserted(flag.unwrap().flag, Cow::Owned(node), Cow::Owned(self.size() + 1)).unwrap())))
            }
        }
    }

    fn remove<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> RemoveResult<'a, B, V, H> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node.remove(value, flag.as_ref().unwrap().next()) {
                RemoveResult::NotFound => RemoveResult::NotFound,
                RemoveResult::Removed(node, reference) => {
                    if node.size() == 0 {
                        RemoveResult::Removed(Arc::new(Self::new(self.nodes.removed(flag.unwrap().flag, Cow::Owned(self.size() - 1)).unwrap())), reference)
                    }
                    else if node.is_cnode() || self.nodes.len() > 1 {
                        RemoveResult::Removed(Arc::new(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(node), Cow::Owned(self.size() - 1)).unwrap())), reference)
                    }
                    else {
                        RemoveResult::Removed(node, reference)
                    }
                }
            },
            Err(_) => RemoveResult::NotFound
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}

impl<B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> Default for CNode<B, V, H> {
    fn default() -> Self {
        CNode::<B, V, H>::new(default_bit_indexed_array())
    }
}
