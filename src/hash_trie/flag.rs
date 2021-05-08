use crate::bit_indexed_array::NthBit;
use alloc::fmt::Debug;
use core::{mem, ops::*};

pub trait LogB<B> {
    fn log_b() -> usize;
}
impl LogB<u8> for u8 { fn log_b() -> usize {3} }
impl LogB<u16> for u16 { fn log_b() -> usize {4} }
impl LogB<u32> for u32 { fn log_b() -> usize {5} }
impl LogB<u64> for u64 { fn log_b() -> usize {6} }
impl LogB<u128> for u128 { fn log_b() -> usize {7} }
impl LogB<usize> for usize {
    fn log_b() -> usize {
        match mem::size_of::<usize>() {
            1 => 3,
            2 => 4,
            4 => 5,
            8 => 6,
            16 => 7,
            _ => panic!()
        }
    }
}

pub trait MaskLogB<B> {
    fn mask_log_b() -> B;
}
impl MaskLogB<u8> for u8 { fn mask_log_b() -> Self {0b111} }
impl MaskLogB<u16> for u16 { fn mask_log_b() -> Self {0b1111} }
impl MaskLogB<u32> for u32 { fn mask_log_b() -> Self {0b11111} }
impl MaskLogB<u64> for u64 { fn mask_log_b() -> Self {0b111111} }
impl MaskLogB<u128> for u128 { fn mask_log_b() -> Self {0b1111111} }
impl MaskLogB<usize> for usize {
    fn mask_log_b() -> usize {
        match mem::size_of::<usize>() {
            1 => 0b111,
            2 => 0b1111,
            4 => 0b11111,
            8 => 0b111111,
            16 => 0b1111111,
            _ => panic!()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Flag <B> {
    hash_value: B,
    depth: usize,
    pub(super) flag: B,
}

impl <B: BitAnd + Clone + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + Shr<usize>> Flag<B> {
    pub(super) fn new(hash_value: B) -> Self {
        Flag {
            depth: 0,
            flag: B::nth_bit(B::from(hash_value.clone().bitand(B::mask_log_b())).into()).unwrap(),
            hash_value,
        }
    }

    pub(super) fn next(&self) -> Option<Flag<B>> {
        if self.depth * B::log_b() >= 8 * mem::size_of::<B>() {
            None
        }
        else {
            Some(Flag {
                hash_value: self.hash_value.clone(),
                depth: self.depth + 1,
                flag: B::nth_bit(B::from(B::from(self.hash_value.clone().shr((self.depth + 1) * B::log_b())).bitand(B::mask_log_b())).into()).unwrap()
            })
        }
    }
}

impl <B: BitAnd + Clone + From<<B as BitAnd>::Output> + From<<B as Shr<usize>>::Output> + Into<usize> + LogB<B> + MaskLogB<B> + NthBit<B> + Shr<usize>> From<B> for Flag<B> {
    fn from(bits: B) -> Flag<B> {
        Self::new(bits)
    }
}