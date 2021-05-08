use crate::bit_indexed_array::*;
use super::{flag::*, mnode::*, snode::*, traits::*};
use alloc::{boxed::Box, borrow::Cow, fmt::{self, Debug, Formatter}, sync::Arc};
use core::ops::*;

#[derive(Clone, Debug)]
pub(super) struct CNode <B, V: Value> {
    nodes: Box<dyn BitIndexedArray::<B, Arc<dyn MNode<B, V>>>>,
    size: usize,
}

impl<B: BitAnd + CountOnes<B> + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + Shr<usize>, V: Value> CNode<B, V> {
    fn new(nodes: Box<dyn BitIndexedArray::<B, Arc<dyn MNode<B, V>>>>, size: usize) -> Self {
        Self { nodes, size }
    }
}

impl <B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> MNode<B, V> for CNode<B, V> {
    fn size(&self) -> usize {
        self.size
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

    fn insert<'a>(&'a self, value: Cow<V>, flag: Option<Flag<B>>) -> InsertResult<'a, B, V> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node.insert(value, flag.as_ref().unwrap().next()) {
                InsertResult::Found(reference) => InsertResult::Found(reference),
                InsertResult::Inserted(node) => InsertResult::Inserted(Arc::new(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(node)).unwrap(), self.size() + 1)))
            },
            Err(_) => {
                let node: Arc::<dyn MNode::<B, V>> = Arc::new(SNode::<V>::new(value.into_owned()));
                InsertResult::Inserted(Arc::new(Self::new(self.nodes.inserted(flag.unwrap().flag, Cow::Owned(node)).unwrap(), self.size() + 1)))
            }
        }
    }

    fn remove<'a>(&'a self, value: &V, flag: Option<Flag<B>>) -> RemoveResult<'a, B, V> {
        match self.nodes.at(flag.as_ref().unwrap().flag.clone()) {
            Ok(node) => match node.remove(value, flag.as_ref().unwrap().next()) {
                RemoveResult::NotFound => RemoveResult::NotFound,
                RemoveResult::Removed(node, reference) => RemoveResult::Removed(Arc::new(Self::new(self.nodes.updated(flag.unwrap().flag, Cow::Owned(node)).unwrap(), self.size() - 1)), reference) // TODO: Implement collapse on last subnode
            },
            Err(_) => RemoveResult::NotFound
        }
    }

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        (&self as &dyn Debug).fmt(formatter)
    }
}

impl<B: BitAnd + BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + NthOne<B> + PartialEq + Shr<usize> + 'static, V: Value> Default for CNode<B, V> {
    fn default() -> Self {
        CNode::<B, V>::new(default_bit_indexed_array(), 0)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn cnode_indices() {
//         assert_eq!(u8::MAX_ENTRIES, 8);
//         assert_eq!(u8::STRIDE, 3);
//         assert_eq!(u8::MASK, 0b111);
//         assert_eq!(u16::MAX_ENTRIES, 16);
//         assert_eq!(u16::STRIDE, 4);
//         assert_eq!(u16::MASK, 0b1111);
//         assert_eq!(u32::MAX_ENTRIES, 32);
//         assert_eq!(u32::STRIDE, 5);
//         assert_eq!(u32::MASK, 0b11111);
//         assert_eq!(u64::MAX_ENTRIES, 64);
//         assert_eq!(u64::STRIDE, 6);
//         assert_eq!(u64::MASK, 0b111111);
//         assert_eq!(u128::MAX_ENTRIES, 128);
//         assert_eq!(u128::STRIDE, 7);
//         assert_eq!(u128::MASK, 0b1111111);
//     }

//     #[test]
//     fn cnode_index_max_entries() {
//         assert_eq!(u8::max_entries::<u8>(0), 3);
//         assert_eq!(u8::max_entries::<u8>(1), 3);
//         assert_eq!(u8::max_entries::<u8>(2), 2);
//         assert_eq!(u8::max_entries::<u8>(3), 0);
//         assert_eq!(u8::max_entries::<u16>(0), 3);
//         assert_eq!(u8::max_entries::<u16>(4), 3);
//         assert_eq!(u8::max_entries::<u16>(5), 1);
//         assert_eq!(u8::max_entries::<u16>(6), 0);
//         assert_eq!(u8::max_entries::<u32>(0), 3);
//         assert_eq!(u8::max_entries::<u32>(9), 3);
//         assert_eq!(u8::max_entries::<u32>(10), 2);
//         assert_eq!(u8::max_entries::<u32>(11), 0);
//         assert_eq!(u8::max_entries::<u64>(0), 3);
//         assert_eq!(u8::max_entries::<u64>(20), 3);
//         assert_eq!(u8::max_entries::<u64>(21), 1);
//         assert_eq!(u8::max_entries::<u64>(22), 0);
//         assert_eq!(u8::max_entries::<u128>(0), 3);
//         assert_eq!(u8::max_entries::<u128>(41), 3);
//         assert_eq!(u8::max_entries::<u128>(42), 2);
//         assert_eq!(u8::max_entries::<u128>(43), 0);
        
//         assert_eq!(u16::max_entries::<u8>(0), 4);
//         assert_eq!(u16::max_entries::<u8>(1), 4);
//         assert_eq!(u16::max_entries::<u8>(2), 0);
//         assert_eq!(u16::max_entries::<u16>(0), 4);
//         assert_eq!(u16::max_entries::<u16>(3), 4);
//         assert_eq!(u16::max_entries::<u16>(4), 0);
//         assert_eq!(u16::max_entries::<u32>(0), 4);
//         assert_eq!(u16::max_entries::<u32>(7), 4);
//         assert_eq!(u16::max_entries::<u32>(8), 0);
//         assert_eq!(u16::max_entries::<u64>(0), 4);
//         assert_eq!(u16::max_entries::<u64>(15), 4);
//         assert_eq!(u16::max_entries::<u64>(16), 0);
//         assert_eq!(u16::max_entries::<u128>(0), 4);
//         assert_eq!(u16::max_entries::<u128>(31), 4);
//         assert_eq!(u16::max_entries::<u128>(32), 0);
        
//         assert_eq!(u32::max_entries::<u8>(0), 5);
//         assert_eq!(u32::max_entries::<u8>(1), 3);
//         assert_eq!(u32::max_entries::<u8>(2), 0);
//         assert_eq!(u32::max_entries::<u16>(0), 5);
//         assert_eq!(u32::max_entries::<u16>(2), 5);
//         assert_eq!(u32::max_entries::<u16>(3), 1);
//         assert_eq!(u32::max_entries::<u16>(5), 0);
//         assert_eq!(u32::max_entries::<u32>(0), 5);
//         assert_eq!(u32::max_entries::<u32>(5), 5);
//         assert_eq!(u32::max_entries::<u32>(6), 2);
//         assert_eq!(u32::max_entries::<u32>(7), 0);
//         assert_eq!(u32::max_entries::<u64>(0), 5);
//         assert_eq!(u32::max_entries::<u64>(11), 5);
//         assert_eq!(u32::max_entries::<u64>(12), 4);
//         assert_eq!(u32::max_entries::<u64>(13), 0);
//         assert_eq!(u32::max_entries::<u128>(0), 5);
//         assert_eq!(u32::max_entries::<u128>(24), 5);
//         assert_eq!(u32::max_entries::<u128>(25), 3);
//         assert_eq!(u32::max_entries::<u128>(26), 0);
        
//         assert_eq!(u64::max_entries::<u8>(0), 6);
//         assert_eq!(u64::max_entries::<u8>(1), 2);
//         assert_eq!(u64::max_entries::<u8>(2), 0);
//         assert_eq!(u64::max_entries::<u16>(0), 6);
//         assert_eq!(u64::max_entries::<u16>(1), 6);
//         assert_eq!(u64::max_entries::<u16>(2), 4);
//         assert_eq!(u64::max_entries::<u16>(3), 0);
//         assert_eq!(u64::max_entries::<u32>(0), 6);
//         assert_eq!(u64::max_entries::<u32>(4), 6);
//         assert_eq!(u64::max_entries::<u32>(5), 2);
//         assert_eq!(u64::max_entries::<u32>(6), 0);
//         assert_eq!(u64::max_entries::<u64>(0), 6);
//         assert_eq!(u64::max_entries::<u64>(9), 6);
//         assert_eq!(u64::max_entries::<u64>(10), 4);
//         assert_eq!(u64::max_entries::<u64>(11), 0);
//         assert_eq!(u64::max_entries::<u128>(0), 6);
//         assert_eq!(u64::max_entries::<u128>(20), 6);
//         assert_eq!(u64::max_entries::<u128>(21), 2);
//         assert_eq!(u64::max_entries::<u128>(23), 0);
        
//         assert_eq!(u128::max_entries::<u8>(0), 7);
//         assert_eq!(u128::max_entries::<u8>(1), 1);
//         assert_eq!(u128::max_entries::<u8>(2), 0);
//         assert_eq!(u128::max_entries::<u16>(0), 7);
//         assert_eq!(u128::max_entries::<u16>(1), 7);
//         assert_eq!(u128::max_entries::<u16>(2), 2);
//         assert_eq!(u128::max_entries::<u16>(3), 0);
//         assert_eq!(u128::max_entries::<u32>(0), 7);
//         assert_eq!(u128::max_entries::<u32>(3), 7);
//         assert_eq!(u128::max_entries::<u32>(4), 4);
//         assert_eq!(u128::max_entries::<u32>(5), 0);
//         assert_eq!(u128::max_entries::<u64>(0), 7);
//         assert_eq!(u128::max_entries::<u64>(8), 7);
//         assert_eq!(u128::max_entries::<u64>(9), 1);
//         assert_eq!(u128::max_entries::<u64>(10), 0);
//         assert_eq!(u128::max_entries::<u128>(0), 7);
//         assert_eq!(u128::max_entries::<u128>(17), 7);
//         assert_eq!(u128::max_entries::<u128>(18), 2);
//         assert_eq!(u128::max_entries::<u128>(19), 0);
//     }
// }
