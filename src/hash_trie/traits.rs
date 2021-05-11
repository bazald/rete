use crate::bit_indexed_array::*;
use super::flag::*;
use alloc::fmt::Debug;
use core::{hash::{Hash, Hasher}, mem, ops::*};

pub trait Bits: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<Self as BitAnd>::Output> + From<<Self as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + Send + Sync + 'static {}
impl <B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<Self as BitAnd>::Output> + From<<Self as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + Send + Sync + 'static> Bits
for B where B: AsUsize + BitAnd + BitContains + BitIndex + BitInsert + BitRemove + Clone + CountOnes + Debug + Default + From<<Self as BitAnd>::Output> + From<<Self as Shr<usize>>::Output> + LogB + MaskLogB + NthBit + NthOne + PartialEq + Shr<usize> + Send + Sync + 'static {}

pub trait Value: Clone + Debug + Eq + PartialEq + Hash + Send + Sync + 'static {}
impl <T: Clone + Debug + Eq + PartialEq + Hash + Send + Sync + 'static> Value
for T where T: Clone + Debug + Eq + PartialEq + Hash + Send + Sync + 'static {}

pub trait HashValuable: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static {}
impl <T: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static> HashValuable
for T where T: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static {}

pub trait HasherBv<B, V>: Clone + Debug + Default + 'static {
    fn hash(&self, value: &V) -> B;
}
impl <V: Clone + Debug + Default + PartialEq + 'static, H: Clone + Debug + Default + Hasher + 'static> HasherBv<u64, V> for H {
    fn hash(&self, value: &V) -> u64 {
        let mut hasher = H::default();
        hasher.write(unsafe { core::slice::from_raw_parts(value as *const V as *const u8, mem::size_of::<V>()) });
        hasher.finish()
    }
}
