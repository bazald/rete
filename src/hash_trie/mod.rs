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

pub struct HashTrie <B, V> {
    root: Arc<dyn MNode<B, V>>,
}

impl <B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> HashTrie<B, V> {
    fn new() -> Self {
        Self {
            root: Arc::new(CNode::<B, V>::default())
        }
    }

    fn singleton(mnode: Arc<dyn MNode<B, V>>) -> Self {
        Self {
            root: mnode
        }
    }

    #[allow(dead_code)]
    fn find<H: HasherBv<B, V>>(&self, value: &V) -> Result<&V, ()> {
        match self.root.find(value, Some(Flag::new(H::default().hash(value)))) {
            FindResult::NotFound => Err(()),
            FindResult::Found(found) => Ok(found)
        }
    }

    #[allow(dead_code)]
    fn insert<H: HasherBv<B, V>>(&self, value: Cow<V>) -> Result<Self, &V> { // TODO: Audit lifetime checks on returned value ref
        let flag = Flag::from(H::default().hash(value.as_ref()));
        match self.root.insert(value, Some(flag)) {
            InsertResult::Found(found) => Err(found),
            InsertResult::Inserted(mnode) => Ok(Self::singleton(mnode))
        }
    }

    #[allow(dead_code)]
    fn remove<H: HasherBv<B, V>>(&self, value: &V) -> Result<(Self, Option<&V>), ()> { // TODO: Audit lifetime checks on returned value ref
        match self.root.remove(value, Some(Flag::from(H::default().hash(value)))) {
            RemoveResult::NotFound => Err(()),
            RemoveResult::Removed(mnode, removed) => Ok((Self::singleton(mnode), Some(removed)))
        }
    }
}

impl <B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> Clone for HashTrie<B, V> {
    fn clone(&self) -> Self {
        Self::singleton(self.root.clone())
    }
}

impl <B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> Default for HashTrie<B, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl <B: Eq, V: Clone + Eq> Eq for HashTrie<B, V> {}

impl <B, V: Clone> PartialEq for HashTrie<B, V> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.root.as_ref() as *const dyn MNode<B, V> as *const u8, other.root.as_ref() as *const dyn MNode<B, V> as *const u8)
    }
}

impl <B: Debug, V: Clone + Debug> Debug for HashTrie<B, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "HashTrie {{ root: {:?} }}", self.root)
    }
}
