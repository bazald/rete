use crate::bit_indexed_array::*;
use super::{cnode::*, flag::*, mnode::*, traits::*};
use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::ops::*;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct SNode<V: Value> {
    pub value: V,
}

#[allow(dead_code)]
impl <V: Value> SNode<V> {
    pub(super) fn new(value: V) -> Self {
        Self {value}
    }

    pub(super) fn get(&self) -> &V {
        &self.value
    }
}

impl <B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> MNode<B, V> for SNode<V> {
    fn size(&self) -> usize {
        1
    }

    fn is_cnode(&self) -> bool {
        false
    }

    fn find<'a>(&'a self, value: &V, _flag: Option<Flag<B>>) -> FindResult<'a, V> {
        if self.value == *value {
            FindResult::Found(&self.value)
        }
        else {
            FindResult::NotFound
        }
    }

    fn insert<'a>(&'a self, value: Cow<V>, _flag: Option<Flag<B>>) -> InsertResult<'a, B, V> {
        if self.value == *value {
            InsertResult::Found(&self.value)
        }
        else {
            let inserted = Self::new(value.into_owned());
            InsertResult::Inserted(Arc::new(inserted)) // TODO: Actually do insertion with CNode/LNode split possibility
        }
    }

    fn remove<'a>(&'a self, value: &V, _flag: Option<Flag<B>>) -> RemoveResult<'a, B, V> {
        if self.value == *value {
            RemoveResult::Removed(Arc::new(CNode::<B, V>::default()), &self.value)
        }
        else {
            RemoveResult::NotFound
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}
