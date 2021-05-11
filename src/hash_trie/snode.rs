use crate::bit_indexed_array::*;
use super::{cnode::*, flag::*, lnode::*, mnode::*, traits::*};
use alloc::{borrow::Cow, fmt::Debug, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SNode<V: Value> {
    value: V,
}

#[allow(dead_code)]
impl <V: Value> SNode<V> {
    pub(super) fn new(value: V) -> Arc<Self> {
        Arc::new(Self {value})
    }

    pub(super) fn get(&self) -> &V {
        &self.value
    }
}

pub(super) enum SNodeInsertResult<B: Bits, V: Value, H: HasherBv<B, V>> {
    InsertedC(CNode<B, V, H>),
    InsertedL(Arc<LNode<V>>),
}

pub(super) fn insert<'a, B: Bits, V: Value, H: HasherBv<B, V>>(this: &'a Arc<SNode<V>>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
    if this.value == *value {
        InsertResult::Found(&this.value)
    }
    else {
        let mut self_flag = if flag.is_some() { Some(Flag::from(H::default().hash(&this.value))) } else { None };
        while self_flag.is_some() && self_flag.as_ref().unwrap().depth() != flag.as_ref().unwrap().depth() {
            self_flag = self_flag.unwrap().next();
        }

        match insert_not_equal(this, self_flag, value, flag) {
            SNodeInsertResult::InsertedC(cnode) => InsertResult::InsertedC(cnode),
            SNodeInsertResult::InsertedL(cnode) => InsertResult::InsertedL(cnode),
        }
    }
}

fn insert_not_equal<B: Bits, V: Value, H: HasherBv<B, V>>(this: &Arc<SNode<V>>, self_flag: Option<Flag<B>>, value: Cow<V>, flag: Option<Flag<B>>) -> SNodeInsertResult<B, V, H> {
    if self_flag.is_none() && flag.is_none() {
        return SNodeInsertResult::InsertedL(LNode::new(value.into_owned(), LNodeNext::S(this.clone())));
    }

    let self_flag = self_flag.unwrap();
    let flag = flag.unwrap();

    if self_flag.flag() != flag.flag() {
        let flags = self_flag.flag().bit_insert(flag.flag()).unwrap();
        let values = if flags.bit_index(self_flag.flag).unwrap() == 0 {
            vec!(MNode::S(this.clone()), MNode::S(SNode::new(value.into_owned())))
        } else {
            vec!(MNode::S(SNode::new(value.into_owned())), MNode::S(this.clone()))
        };
        SNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(flags, values, 2_usize).unwrap()))
    }
    else {
        match insert_not_equal(this, self_flag.next(), value, flag.next()) {
            SNodeInsertResult::InsertedC(cnode) => SNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::C(cnode)), 1_usize).unwrap())),
            SNodeInsertResult::InsertedL(lnode) => SNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::L(lnode)), 1_usize).unwrap())),
        }
    }
}
