use alloc::fmt::Debug;
use core::{hash::Hash, ops::*};

pub trait Value: Clone + Debug + Eq + PartialEq + Hash + 'static {}
impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> Value
for T where T: Clone + Debug + Eq + PartialEq + Hash + 'static {}

pub trait HashValuable: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static {}
impl <T: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static> HashValuable
for T where T: Sized + Clone + Debug + Default + Eq + Ord + BitAnd + BitOr + BitXor + From<u8> + Ord + Shl<usize> + Shr<usize> + Sub + SubAssign + 'static {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct HashValue <T: HashValuable + From<<T as BitAnd>::Output> + From<<T as BitOr>::Output> + From<<T as BitXor>::Output> + From<<T as Shl<usize>>::Output> + From<<T as Shr<usize>>::Output> + From<<T as Sub>::Output>>
{
    hash_value: T,
}

impl <T: HashValuable + From<<T as BitAnd>::Output> + From<<T as BitOr>::Output> + From<<T as BitXor>::Output> + From<<T as Shl<usize>>::Output> + From<<T as Shr<usize>>::Output> + From<<T as Sub>::Output>> HashValue<T> {
    pub fn new(hash_value: T) -> Self {
        Self {
            hash_value
        }
    }

    #[allow(unused)]
    pub fn get(&self) -> T {
        self.hash_value.clone()
    }
}

impl <T: HashValuable + From<<T as BitAnd>::Output> + From<<T as BitOr>::Output> + From<<T as BitXor>::Output> + From<<T as Shl<usize>>::Output> + From<<T as Shr<usize>>::Output> + From<<T as Sub>::Output>> Default for HashValue<T> {
    fn default() -> Self {
        Self::new(0_u8.into())
    }
}

impl <T: HashValuable + From<<T as BitAnd>::Output> + From<<T as BitOr>::Output> + From<<T as BitXor>::Output> + From<<T as Shl<usize>>::Output> + From<<T as Shr<usize>>::Output> + From<<T as Sub>::Output>> From<T> for HashValue<T> {
    fn from(hash_value: T) -> Self {
        Self::new(hash_value)
    }
}

pub trait HasherBv<B, V>: Debug + Default {
    fn hash(&mut self, value: &V) -> B;
}
