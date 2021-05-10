mod flag;
mod traits;
#[macro_use]
mod mnode;
mod cnode;
mod lnode;
mod snode;

use crate::bit_indexed_array::*;
use cnode::*;
use flag::*;
use mnode::*;
use traits::*;

use alloc::{borrow::Cow, fmt::{Debug, Formatter}, sync::Arc};
use core::{ops::*, ptr};

pub struct HashTrie <B, V, H> {
    root: ArcMNode<B, V, H>,
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> HashTrie<B, V, H> {
    fn new() -> Self {
        Self {
            root: Arc::new(CNode::<B, V, H>::default())
        }
    }

    fn singleton(mnode: ArcMNode<B, V, H>) -> Self {
        Self {
            root: mnode
        }
    }

    #[allow(dead_code)]
    fn find(&self, value: &V) -> Result<&V, ()> {
        match self.root.find(value, Some(Flag::new(H::default().hash(value)))) {
            FindResult::NotFound => Err(()),
            FindResult::Found(found) => Ok(found)
        }
    }

    #[allow(dead_code)]
    fn insert(&self, value: Cow<V>) -> Result<Self, &V> {
        let flag = Flag::from(H::default().hash(value.as_ref()));
        match self.root.insert(self.root.clone(), value, Some(flag)) {
            InsertResult::Found(found) => Err(found),
            InsertResult::Inserted(mnode) => Ok(Self::singleton(mnode))
        }
    }

    #[allow(dead_code)]
    fn remove(&self, value: &V) -> Result<(Self, Option<&V>), ()> {
        match self.root.remove(value, Some(Flag::from(H::default().hash(value)))) {
            RemoveResult::NotFound => Err(()),
            RemoveResult::Removed(mnode, removed) => Ok((Self::singleton(mnode), Some(removed)))
        }
    }
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> Clone for HashTrie<B, V, H> {
    fn clone(&self) -> Self {
        Self::singleton(self.root.clone())
    }
}

impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + 'static, V: Value, H: HasherBv<B, V>> Default for HashTrie<B, V, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl <B: Eq, V: Clone + Eq, H: HasherBv<B, V>> Eq for HashTrie<B, V, H> {}

impl <B, V: Clone, H: HasherBv<B, V>> PartialEq for HashTrie<B, V, H> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.root.as_ref() as *const dyn MNode<B, V, H> as *const u8, other.root.as_ref() as *const dyn MNode<B, V, H> as *const u8)
    }
}

impl <B: Debug, V: Clone + Debug, H: HasherBv<B, V>> Debug for HashTrie<B, V, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "HashTrie {{ root: {:?} }}", self.root)
    }
}
