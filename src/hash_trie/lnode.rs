use crate::bit_indexed_array::*;
use super::{cnode::*, flag::*, mnode::*, traits::*};
use alloc::{borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::ops::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LNode<V: Value> {
    pub value: V,
    pub next: Option<Arc<LNode<V>>>,
    pub size: usize,
}

pub(super) enum LnodeRemoveResult<'a, V: Value> {
    NotFound,
    Removed(Option<Arc<LNode<V>>>, &'a V),
}

#[allow(dead_code)]
impl<V: Value> LNode<V> {
    pub(super) fn new(value: V, next: Option<Arc<Self>>) -> Arc<Self> {
        let size = 1 + match &next {
            Some(next) => next.size,
            None => 0
        };
        Arc::new(Self {
            value,
            next,
            size,
        })
    }
    
    fn remove_from_lnode<'a>(&'a self, value: &V) -> LnodeRemoveResult<'a, V> {
        if self.value == *value {
            return LnodeRemoveResult::Removed(self.next.clone(), &self.value);
        }
        match &self.next {
            Some(next) => match next.remove_from_lnode(value) {
                LnodeRemoveResult::Removed(next, reference) => LnodeRemoveResult::Removed(Some(LNode::new(self.value.clone(), next)), reference),
                LnodeRemoveResult::NotFound => LnodeRemoveResult::NotFound
            },
            None => LnodeRemoveResult::NotFound
        }
    }
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> MNode<B, V, H> for LNode<V> {
    fn size(&self) -> usize {
        self.size
    }

    fn is_cnode(&self) -> bool {
        false
    }

    fn find<'a>(&'a self, value: &V, _flag: Option<Flag<B>>) -> FindResult<'a, V> {
        if self.value == *value {
            FindResult::Found(&self.value)
        }
        else {
            match &self.next {
                Some(next) => (next.as_ref() as &dyn MNode<B, V, H>).find(value, Option::<Flag<B>>::None),
                None => FindResult::NotFound
            }
        }
    }

    fn insert<'a>(&'a self, arc_self: ArcMNode<B, V, H>, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V, H> {
        match (self as &dyn MNode<B, V, H>).find(value.as_ref(), Option::<Flag<B>>::None) {
            FindResult::Found(found) => InsertResult::Found(found),
            FindResult::NotFound => {
                let lnode = LNode::new(value.into_owned(), self.next.clone());
                InsertResult::Inserted(lnode) // TODO: Actually do insertion with CNode split possibility
            }
        }
    }

    fn remove<'a>(&'a self, value: &V, _flag: Option<Flag<B>>) -> RemoveResult<'a, B, V, H> {
        match self.remove_from_lnode(value) {
            LnodeRemoveResult::NotFound => RemoveResult::NotFound,
            LnodeRemoveResult::Removed(node, reference) => RemoveResult::Removed(match node {
                Some(node) => node,
                None => Arc::new(CNode::<B, V, H>::default())
            }, reference)
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}

#[allow(unused_macros)]
macro_rules! lnode {
    ( $value:expr ) => {
        {
            LNode::new($value, None)
        }
    };
    ( $value:expr, $($rest:expr),+ ) => {
        {
            LNode::new($value, Some(lnode!($($rest),*)))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn lnode_insert_3() {
        let ln = lnode!(3, 2, 1);
        let node: Arc<dyn MNode<u64, i32, DefaultHasher>> = ln.clone();
        assert_eq!(node.size(), 3);
        assert_found_eq!(node.find(&1, Option::<Flag<u64>>::None), 1);
        assert_found_eq!(node.find(&2, Option::<Flag<u64>>::None), 2);
        assert_found_eq!(node.find(&3, Option::<Flag<u64>>::None), 3);
        assert_found_none!(node.find(&4, Option::<Flag<u64>>::None));
    }

    #[test]
    fn lnode_insert_3_again() {
        let ln = lnode!(3, 2, 1);
        let node: Arc<dyn MNode<u64, i32, DefaultHasher>> = ln.clone();
        match node.insert(node.clone(), Cow::Owned(3), Option::<Flag<u64>>::None) {
            InsertResult::Found(v) => assert_eq!(*v, 3),
            InsertResult::Inserted(_) => panic!()
        }
    }

    #[test]
    fn lnode_remove_1() {
        match (lnode!(3, 2, 1).as_ref() as &dyn MNode<u64, i32, DefaultHasher>).remove(&1, Option::<Flag<u64>>::None) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::Removed(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_none!(ln.find(&1, Option::<Flag<u64>>::None));
                assert_found_eq!(ln.find(&2, Option::<Flag<u64>>::None), 2);
                assert_found_eq!(ln.find(&3, Option::<Flag<u64>>::None), 3);
                assert_found_none!(ln.find(&4, Option::<Flag<u64>>::None));
            }
        }
    }

    #[test]
    fn lnode_remove_2() {
        match (lnode!(3, 2, 1).as_ref() as &dyn MNode<u64, i32, DefaultHasher>).remove(&2, Option::<Flag<u64>>::None) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::Removed(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_eq!(ln.find(&1, Option::<Flag<u64>>::None), 1);
                assert_found_none!(ln.find(&2, Option::<Flag<u64>>::None));
                assert_found_eq!(ln.find(&3, Option::<Flag<u64>>::None), 3);
                assert_found_none!(ln.find(&4, Option::<Flag<u64>>::None));
            }
        }
    }

    #[test]
    fn lnode_remove_3() {
        match (lnode!(3, 2, 1).as_ref() as &dyn MNode<u64, i32, DefaultHasher>).remove(&3, Option::<Flag<u64>>::None) {
            RemoveResult::NotFound => panic!(),
            RemoveResult::Removed(ln, _) => {
                assert_eq!(ln.size(), 2);
                assert_found_eq!(ln.find(&1, Option::<Flag<u64>>::None), 1);
                assert_found_eq!(ln.find(&2, Option::<Flag<u64>>::None), 2);
                assert_found_none!(ln.find(&3, Option::<Flag<u64>>::None));
                assert_found_none!(ln.find(&4, Option::<Flag<u64>>::None));
            }
        }
    }
}
