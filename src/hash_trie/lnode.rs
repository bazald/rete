use crate::bit_indexed_array::*;
use super::{cnode::*, flag::*, mnode::*, snode::{self, *}, traits::*};
use alloc::{borrow::Cow, fmt::Debug, sync::*};

#[derive(Clone, Debug)]
pub(super) enum LNodeNext<V: Value> {
    L(Arc<LNode<V>>),
    S(Arc<SNode<V>>),
}

#[derive(Clone, Debug)]
pub(super) struct LNode<V: Value> {
    value: V,
    next: LNodeNext<V>,
    size: usize,
}

pub(super) enum LNodeRemoveResult<'a, V: Value> {
    NotFound,
    RemovedL(Arc<LNode<V>>, &'a V),
    RemovedS(Arc<SNode<V>>, &'a V),
}

#[allow(dead_code)]
impl<V: Value> LNode<V> {
    pub(super) fn new(value: V, next: LNodeNext<V>) -> Arc<Self> {
        let size = 1 + match &next {
            LNodeNext::L(lnode) => lnode.size,
            LNodeNext::S(_snode) => 1,
        };
        Arc::new(Self {
            value,
            next,
            size,
        })
    }

    pub(super) fn get(&self) -> &V {
        &self.value
    }
    
    pub(super) fn next(&self) -> &LNodeNext<V> {
        &self.next
    }
    
    pub(super) fn size(&self) -> usize {
        self.size
    }

    pub(super) fn find<'a>(&'a self, value: &V) -> FindResult<'a, V> {
        if self.value == *value {
            FindResult::Found(&self.value)
        }
        else {
            match &self.next {
                LNodeNext::L(lnode) => lnode.find(value),
                LNodeNext::S(snode) => {
                    if *snode.get() == *value {
                        FindResult::Found(snode.get())
                    }
                    else {
                        FindResult::NotFound
                    }
                },
            }
        }
    }

    pub(super) fn remove<'a, B: Bits, H: HasherBv<B, V>>(&'a self, value: &V) -> RemoveResult<'a, B, V, H> {
        match self.remove_from_lnode(value) {
            LNodeRemoveResult::NotFound => RemoveResult::NotFound,
            LNodeRemoveResult::RemovedL(lnode, reference) => RemoveResult::RemovedL(lnode, reference),
            LNodeRemoveResult::RemovedS(snode, reference) => RemoveResult::RemovedS(snode, reference),
        }
    }
    
    fn remove_from_lnode<'a>(&'a self, value: &V) -> LNodeRemoveResult<'a, V> {
        if self.value == *value {
            match &self.next {
                LNodeNext::L(lnode) => LNodeRemoveResult::RemovedL(lnode.clone(), &self.value),
                LNodeNext::S(snode) => LNodeRemoveResult::RemovedS(snode.clone(), &self.value),
            }
        }
        else {
            match &self.next {
                LNodeNext::L(lnode) => match lnode.remove_from_lnode(value) {
                    LNodeRemoveResult::NotFound => LNodeRemoveResult::NotFound,
                    LNodeRemoveResult::RemovedL(lnode, reference) => LNodeRemoveResult::RemovedL(lnode, reference),
                    LNodeRemoveResult::RemovedS(snode, reference) => LNodeRemoveResult::RemovedS(snode, reference),
                },
                LNodeNext::S(snode) => {
                    if *snode.get() == *value {
                        LNodeRemoveResult::RemovedS(SNode::new(self.value.clone()), snode.get())
                    }
                    else {
                        LNodeRemoveResult::NotFound
                    }
                }
            }
        }
    }
}

pub(super) enum LNodeInsertResult<B: Bits, V: Value, H: HasherBv<B, V>> {
    InsertedC(CNode<B, V, H>),
    InsertedL(Arc<LNode<V>>),
}

pub(super) fn insert<'a, B: Bits, V: Value, H: HasherBv<B, V>>(this: &'a Arc<LNode<V>>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
    match this.find(value.as_ref()) {
        FindResult::Found(found) => InsertResult::Found(found),
        FindResult::NotFound => {
            let mut self_flag = if flag.is_some() { Some(Flag::from(H::default().hash(&this.value))) } else { None };
            while self_flag.is_some() && self_flag.as_ref().unwrap().depth() != flag.as_ref().unwrap().depth() {
                self_flag = self_flag.unwrap().next();
            }

            match insert_not_found(this, self_flag, value, flag) {
                LNodeInsertResult::InsertedC(cnode) => InsertResult::InsertedC(cnode),
                LNodeInsertResult::InsertedL(lnode) => InsertResult::InsertedL(lnode),
            }
        }
    }
}

fn insert_not_found<B: Bits, V: Value, H: HasherBv<B, V>>(this: &Arc<LNode<V>>, self_flag: Option<Flag<B>>, value: Cow<V>, flag: Option<Flag<B>>) -> LNodeInsertResult<B, V, H> {
    if self_flag.is_none() && flag.is_none() {
        return LNodeInsertResult::InsertedL(LNode::new(value.into_owned(), LNodeNext::L(this.clone())));
    }

    let self_flag = self_flag.unwrap();
    let flag = flag.unwrap();

    if self_flag.flag() != flag.flag() {
        let flags = self_flag.flag().bit_insert(flag.flag()).unwrap();
        let values = if flags.bit_index(self_flag.flag).unwrap() == 0 {
            vec!(MNode::L(this.clone()), MNode::S(SNode::new(value.into_owned())))
        } else {
            vec!(MNode::S(SNode::new(value.into_owned())), MNode::L(this.clone()))
        };
        LNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(flags, values, 2_usize).unwrap()))
    }
    else {
        match &this.next {
            LNodeNext::L(lnode) => match insert_not_found(lnode, self_flag.next(), value, flag.next()) {
                LNodeInsertResult::InsertedC(cnode) => LNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::C(cnode)), 1_usize).unwrap())),
                LNodeInsertResult::InsertedL(lnode) => LNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::L(lnode)), 1_usize).unwrap())),
            },
            LNodeNext::S(snode) => match snode::insert(snode, value, self_flag.next()) {
                InsertResult::Found(_reference) => panic!(),
                InsertResult::InsertedC(cnode) => LNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::C(cnode)), 1_usize).unwrap())),
                InsertResult::InsertedL(lnode) => LNodeInsertResult::InsertedC(CNode::new(new_bit_indexed_array(self_flag.flag(), vec!(MNode::L(lnode)), 1_usize).unwrap())),
            },
        }
    }
}

#[allow(unused_macros)]
macro_rules! lnode {
    ( $value:expr, $snode:expr ) => {
        {
            LNode::new($value, LNodeNext::S(SNode::new($snode)))
        }
    };
    ( $value:expr, $($rest:expr),+ ) => {
        {
            LNode::new($value, LNodeNext::L(lnode!($($rest),*)))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    
    #[test]
    fn lnode_insert_3() {
        let node = lnode!(3, 2, 1);
        assert_eq!(node.size(), 3);
        assert_found_eq!(node.find(&1), 1);
        assert_found_eq!(node.find(&2), 2);
        assert_found_eq!(node.find(&3), 3);
        assert_found_none!(node.find(&4));
    }

    #[test]
    fn lnode_insert_3_again() {
        let node = lnode!(3, 2, 1);
        match insert::<u64, i32, DefaultHasher>(&node, Cow::Owned(3), Option::<Flag<u64>>::None) {
            InsertResult::Found(v) => assert_eq!(*v, 3),
            InsertResult::InsertedC(_) => panic!(),
            InsertResult::InsertedL(_) => panic!(),
        }
    }

    #[test]
    fn lnode_remove_1() {
        match lnode!(3, 2, 1).as_ref().remove::<u64, DefaultHasher>(&1) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::RemovedC(_cnode, _reference) => panic!(),
            RemoveResult::RemovedL(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_none!(ln.find(&1));
                assert_found_eq!(ln.find(&2), 2);
                assert_found_eq!(ln.find(&3), 3);
                assert_found_none!(ln.find(&4));
            },
            RemoveResult::RemovedS(_snode, _reference) => panic!(),
            RemoveResult::RemovedZ(_reference) => panic!(),
        }
    }

    #[test]
    fn lnode_remove_2() {
        match lnode!(3, 2, 1).as_ref().remove::<u64, DefaultHasher>(&2) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::RemovedC(_cnode, _reference) => panic!(),
            RemoveResult::RemovedL(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_eq!(ln.find(&1), 1);
                assert_found_none!(ln.find(&2));
                assert_found_eq!(ln.find(&3), 3);
                assert_found_none!(ln.find(&4));
            },
            RemoveResult::RemovedS(_snode, _reference) => panic!(),
            RemoveResult::RemovedZ(_reference) => panic!(),
        }
    }

    #[test]
    fn lnode_remove_3() {
        match lnode!(3, 2, 1).as_ref().remove::<u64, DefaultHasher>(&3) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::RemovedC(_cnode, _reference) => panic!(),
            RemoveResult::RemovedL(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_eq!(ln.find(&1), 1);
                assert_found_eq!(ln.find(&2), 2);
                assert_found_none!(ln.find(&3));
                assert_found_none!(ln.find(&4));
            },
            RemoveResult::RemovedS(_snode, _reference) => panic!(),
            RemoveResult::RemovedZ(_reference) => panic!(),
        }
    }
}
