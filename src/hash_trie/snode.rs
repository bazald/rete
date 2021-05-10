use crate::bit_indexed_array::*;
use super::{cnode::*, flag::*, lnode::*, mnode::*, traits::*};
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

    fn insert_not_equal<'a, B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, H: HasherBv<B, V>>(&'a self, arc_self: ArcMNode<B, V, H>, self_flag: Option<Flag<B>>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
        if self_flag.is_none() && flag.is_none() {
            return InsertResult::Inserted(LNode::new(value.into_owned(), Some(LNode::new(self.value.clone(), None))));
        }

        let self_flag = self_flag.unwrap();
        let flag = flag.unwrap();

        if self_flag.flag() != flag.flag() {
            let flags = self_flag.flag().bit_insert(flag.flag()).unwrap();
            let arc_self: ArcMNode<B, V, H> = Arc::new(SNode::new(self.value.clone()));
            let arc_value: ArcMNode<B, V, H> = Arc::new(SNode::new(value.into_owned()));
            let values = if flags.bit_index(self_flag.flag).unwrap() == 0 {
                vec!(arc_self, arc_value)
            } else {
                vec!(arc_value, arc_self)
            };
            InsertResult::Inserted(Arc::new(CNode::new(new_bit_indexed_array(flags, values, 2_usize).unwrap())))
        }
        else {
            match self.insert_not_equal(arc_self, self_flag.next(), value, flag.next()) {
                InsertResult::Found(_) => panic!(),
                InsertResult::Inserted(node) => InsertResult::Inserted(Arc::new(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(node), 1_usize).unwrap())))
            }
        }
    }
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> MNode<B, V, H> for SNode<V> {
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

    fn insert<'a>(&'a self, arc_self: ArcMNode<B, V, H>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
        if self.value == *value {
            InsertResult::Found(&self.value)
        }
        else {
            let mut self_flag = if flag.is_some() { Some(Flag::from(H::default().hash(self.get()))) } else { None };
            while self_flag.is_some() && self_flag.as_ref().unwrap().depth() != flag.as_ref().unwrap().depth() {
                self_flag = self_flag.unwrap().next();
            }
            self.insert_not_equal(arc_self, self_flag, value, flag)
        }
    }

    fn remove<'a>(&'a self, value: &V, _flag: Option<Flag<B>>) -> RemoveResult<'a, B, V, H> {
        if self.value == *value {
            RemoveResult::Removed(Arc::new(CNode::<B, V, H>::default()), &self.value)
        }
        else {
            RemoveResult::NotFound
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}
