use super::mnode::MNode;
use alloc::sync::Arc;
use core::{cmp::min, hash::Hash, mem::size_of, ops::*};

const fn log2(value: u128) -> u128 {
    let mut result: u32 = 1;
    while 2_u128.wrapping_pow(result) != value {
        result += 1;
    }
    result as u128
}

const fn mask(mut stride: u128) -> u128 {
    let mut result: u128 = 0;
    while stride > 0 {
        result = (result << 1) | 1;
        stride -= 1;
    }
    result
}

trait CNodeIndex: Sized + BitAnd + BitOr + Shl + Shr {
    const MAX_ENTRIES: u128;
    const STRIDE: u128;
    const MASK: u128;

    fn shift_right(self, shift: u128) -> u128;

    fn max_entries<HashValue: CNodeIndex>(depth: usize) -> usize;
    fn num_entries(&self) -> usize;
    fn entry_index<HashValue: CNodeIndex>(&self, depth: usize, hash: HashValue) -> Option<usize>;
}

macro_rules! index {
    ( $t:ty ) => {
        impl CNodeIndex for $t {
            const MAX_ENTRIES: u128 = 8_u128 * size_of::<$t>() as u128;
            const STRIDE: u128 = log2(Self::MAX_ENTRIES);
            const MASK: u128 = mask(Self::STRIDE);

            fn shift_right(self, shift: u128) -> u128 {
                self as u128 >> shift
            }

            fn max_entries<HashValue: CNodeIndex>(depth: usize) -> usize {
                if depth as u128 * Self::STRIDE >= HashValue::MAX_ENTRIES {
                    0
                } else {
                    min(
                        Self::STRIDE,
                        HashValue::MAX_ENTRIES - depth as u128 * Self::STRIDE,
                    ) as usize
                }
            }

            fn num_entries(&self) -> usize {
                self.count_ones() as usize
            }

            fn entry_index<HashValue: CNodeIndex>(
                &self,
                depth: usize,
                hash: HashValue,
            ) -> Option<usize> {
                if depth as u128 * Self::STRIDE >= HashValue::MAX_ENTRIES {
                    return None;
                }
                Some((hash.shift_right(depth as u128 * Self::STRIDE) & Self::MASK) as usize)
            }
        }
    };
}

index!(u8);
index!(u16);
index!(u32);
index!(u64);
index!(u128);
index!(usize);

trait CNode {}

struct CNodeImpl<T: Clone + Eq + PartialEq + Hash, IndexType: CNodeIndex, const SIZE: usize> {
    index: IndexType,
    nodes: [Arc<dyn MNode<T>>; SIZE],
}

impl<T: Clone + Eq + PartialEq + Hash, Index: CNodeIndex, const SIZE: usize> CNodeImpl<T, Index, SIZE> {
    fn new(index: Index, nodes: [Arc<dyn MNode<T>>; SIZE]) -> Result<Self, ()> {
        if index.num_entries() != SIZE {
            return Err(());
        }
        Ok(Self { index, nodes })
    }
}

impl<T: Clone + Eq + PartialEq + Hash, Index: CNodeIndex, const SIZE: usize> CNode
    for CNodeImpl<T, Index, SIZE>
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cnode_indices() {
        assert_eq!(u8::MAX_ENTRIES, 8);
        assert_eq!(u8::STRIDE, 3);
        assert_eq!(u8::MASK, 0b111);
        assert_eq!(u16::MAX_ENTRIES, 16);
        assert_eq!(u16::STRIDE, 4);
        assert_eq!(u16::MASK, 0b1111);
        assert_eq!(u32::MAX_ENTRIES, 32);
        assert_eq!(u32::STRIDE, 5);
        assert_eq!(u32::MASK, 0b11111);
        assert_eq!(u64::MAX_ENTRIES, 64);
        assert_eq!(u64::STRIDE, 6);
        assert_eq!(u64::MASK, 0b111111);
        assert_eq!(u128::MAX_ENTRIES, 128);
        assert_eq!(u128::STRIDE, 7);
        assert_eq!(u128::MASK, 0b1111111);
    }

    #[test]
    fn cnode_index_max_entries() {
        assert_eq!(u8::max_entries::<u8>(0), 3);
        assert_eq!(u8::max_entries::<u8>(1), 3);
        assert_eq!(u8::max_entries::<u8>(2), 2);
        assert_eq!(u8::max_entries::<u8>(3), 0);
        assert_eq!(u8::max_entries::<u16>(0), 3);
        assert_eq!(u8::max_entries::<u16>(4), 3);
        assert_eq!(u8::max_entries::<u16>(5), 1);
        assert_eq!(u8::max_entries::<u16>(6), 0);
        assert_eq!(u8::max_entries::<u32>(0), 3);
        assert_eq!(u8::max_entries::<u32>(9), 3);
        assert_eq!(u8::max_entries::<u32>(10), 2);
        assert_eq!(u8::max_entries::<u32>(11), 0);
        assert_eq!(u8::max_entries::<u64>(0), 3);
        assert_eq!(u8::max_entries::<u64>(20), 3);
        assert_eq!(u8::max_entries::<u64>(21), 1);
        assert_eq!(u8::max_entries::<u64>(22), 0);
        assert_eq!(u8::max_entries::<u128>(0), 3);
        assert_eq!(u8::max_entries::<u128>(41), 3);
        assert_eq!(u8::max_entries::<u128>(42), 2);
        assert_eq!(u8::max_entries::<u128>(43), 0);
        
        assert_eq!(u16::max_entries::<u8>(0), 4);
        assert_eq!(u16::max_entries::<u8>(1), 4);
        assert_eq!(u16::max_entries::<u8>(2), 0);
        assert_eq!(u16::max_entries::<u16>(0), 4);
        assert_eq!(u16::max_entries::<u16>(3), 4);
        assert_eq!(u16::max_entries::<u16>(4), 0);
        assert_eq!(u16::max_entries::<u32>(0), 4);
        assert_eq!(u16::max_entries::<u32>(7), 4);
        assert_eq!(u16::max_entries::<u32>(8), 0);
        assert_eq!(u16::max_entries::<u64>(0), 4);
        assert_eq!(u16::max_entries::<u64>(15), 4);
        assert_eq!(u16::max_entries::<u64>(16), 0);
        assert_eq!(u16::max_entries::<u128>(0), 4);
        assert_eq!(u16::max_entries::<u128>(31), 4);
        assert_eq!(u16::max_entries::<u128>(32), 0);
        
        assert_eq!(u32::max_entries::<u8>(0), 5);
        assert_eq!(u32::max_entries::<u8>(1), 3);
        assert_eq!(u32::max_entries::<u8>(2), 0);
        assert_eq!(u32::max_entries::<u16>(0), 5);
        assert_eq!(u32::max_entries::<u16>(2), 5);
        assert_eq!(u32::max_entries::<u16>(3), 1);
        assert_eq!(u32::max_entries::<u16>(5), 0);
        assert_eq!(u32::max_entries::<u32>(0), 5);
        assert_eq!(u32::max_entries::<u32>(5), 5);
        assert_eq!(u32::max_entries::<u32>(6), 2);
        assert_eq!(u32::max_entries::<u32>(7), 0);
        assert_eq!(u32::max_entries::<u64>(0), 5);
        assert_eq!(u32::max_entries::<u64>(11), 5);
        assert_eq!(u32::max_entries::<u64>(12), 4);
        assert_eq!(u32::max_entries::<u64>(13), 0);
        assert_eq!(u32::max_entries::<u128>(0), 5);
        assert_eq!(u32::max_entries::<u128>(24), 5);
        assert_eq!(u32::max_entries::<u128>(25), 3);
        assert_eq!(u32::max_entries::<u128>(26), 0);
        
        assert_eq!(u64::max_entries::<u8>(0), 6);
        assert_eq!(u64::max_entries::<u8>(1), 2);
        assert_eq!(u64::max_entries::<u8>(2), 0);
        assert_eq!(u64::max_entries::<u16>(0), 6);
        assert_eq!(u64::max_entries::<u16>(1), 6);
        assert_eq!(u64::max_entries::<u16>(2), 4);
        assert_eq!(u64::max_entries::<u16>(3), 0);
        assert_eq!(u64::max_entries::<u32>(0), 6);
        assert_eq!(u64::max_entries::<u32>(4), 6);
        assert_eq!(u64::max_entries::<u32>(5), 2);
        assert_eq!(u64::max_entries::<u32>(6), 0);
        assert_eq!(u64::max_entries::<u64>(0), 6);
        assert_eq!(u64::max_entries::<u64>(9), 6);
        assert_eq!(u64::max_entries::<u64>(10), 4);
        assert_eq!(u64::max_entries::<u64>(11), 0);
        assert_eq!(u64::max_entries::<u128>(0), 6);
        assert_eq!(u64::max_entries::<u128>(20), 6);
        assert_eq!(u64::max_entries::<u128>(21), 2);
        assert_eq!(u64::max_entries::<u128>(23), 0);
        
        assert_eq!(u128::max_entries::<u8>(0), 7);
        assert_eq!(u128::max_entries::<u8>(1), 1);
        assert_eq!(u128::max_entries::<u8>(2), 0);
        assert_eq!(u128::max_entries::<u16>(0), 7);
        assert_eq!(u128::max_entries::<u16>(1), 7);
        assert_eq!(u128::max_entries::<u16>(2), 2);
        assert_eq!(u128::max_entries::<u16>(3), 0);
        assert_eq!(u128::max_entries::<u32>(0), 7);
        assert_eq!(u128::max_entries::<u32>(3), 7);
        assert_eq!(u128::max_entries::<u32>(4), 4);
        assert_eq!(u128::max_entries::<u32>(5), 0);
        assert_eq!(u128::max_entries::<u64>(0), 7);
        assert_eq!(u128::max_entries::<u64>(8), 7);
        assert_eq!(u128::max_entries::<u64>(9), 1);
        assert_eq!(u128::max_entries::<u64>(10), 0);
        assert_eq!(u128::max_entries::<u128>(0), 7);
        assert_eq!(u128::max_entries::<u128>(17), 7);
        assert_eq!(u128::max_entries::<u128>(18), 2);
        assert_eq!(u128::max_entries::<u128>(19), 0);
    }
}
