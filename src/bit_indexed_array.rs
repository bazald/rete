use alloc::{borrow::Cow, boxed::Box, collections::VecDeque, fmt::{Debug, Formatter}, vec::Vec};
use core::{mem, ptr};

pub trait BitContains<T> {
    fn bit_contains(&self, bit: Self) -> Result<bool, ()>;
}
impl BitContains<u8> for u8 { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }
impl BitContains<u16> for u16 { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }
impl BitContains<u32> for u32 { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }
impl BitContains<u64> for u64 { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }
impl BitContains<u128> for u128 { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }
impl BitContains<usize> for usize { fn bit_contains(&self, bit: Self) -> Result<bool, ()> {if bit.count_ones() == 1 {Ok((self & bit) != 0)} else {Err(())}} }

pub trait BitIndex<T> {
    fn bit_index(&self, bit: Self) -> Result<usize, ()>;
}
impl BitIndex<u8> for u8 { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }
impl BitIndex<u16> for u16 { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }
impl BitIndex<u32> for u32 { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }
impl BitIndex<u64> for u64 { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }
impl BitIndex<u128> for u128 { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }
impl BitIndex<usize> for usize { fn bit_index(&self, bit: Self) -> Result<usize, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok((self & (bit - 1)).count_ones() as usize)} else {Err(())}} }

pub trait BitInsert<T> {
    fn bit_insert(&self, bit: Self) -> Result<Self, ()> where Self: Sized;
}
impl BitInsert<u8> for u8 { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }
impl BitInsert<u16> for u16 { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }
impl BitInsert<u32> for u32 { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }
impl BitInsert<u64> for u64 { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }
impl BitInsert<u128> for u128 { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }
impl BitInsert<usize> for usize { fn bit_insert(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit == 0 {Ok(self | bit)} else {Err(())}} }

pub trait BitRemove<T> {
    fn bit_remove(&self, bit: Self) -> Result<Self, ()> where Self: Sized;
}
impl BitRemove<u8> for u8 { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }
impl BitRemove<u16> for u16 { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }
impl BitRemove<u32> for u32 { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }
impl BitRemove<u64> for u64 { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }
impl BitRemove<u128> for u128 { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }
impl BitRemove<usize> for usize { fn bit_remove(&self, bit: Self) -> Result<Self, ()> {if bit.count_ones() == 1 && self & bit != 0 {Ok(self ^ bit)} else {Err(())}} }

pub trait CountOnes<T> {
    fn count_ones_t(&self) -> usize;
}
impl CountOnes<u8> for u8 { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }
impl CountOnes<u16> for u16 { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }
impl CountOnes<u32> for u32 { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }
impl CountOnes<u64> for u64 { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }
impl CountOnes<u128> for u128 { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }
impl CountOnes<usize> for usize { fn count_ones_t(&self) -> usize {self.count_ones() as usize} }

pub trait NthBit<T> {
    fn nth_bit(n: usize) -> Result<Self, ()> where Self: Sized;
}
impl NthBit<u8> for u8 { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_u8 << n)} else {Err(())}} }
impl NthBit<u16> for u16 { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_u16 << n)} else {Err(())}} }
impl NthBit<u32> for u32 { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_u32 << n)} else {Err(())}} }
impl NthBit<u64> for u64 { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_u64 << n)} else {Err(())}} }
impl NthBit<u128> for u128 { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_u128 << n)} else {Err(())}} }
impl NthBit<usize> for usize { fn nth_bit(n: usize) -> Result<Self, ()> {if n < 8 {Ok(1_usize << n)} else {Err(())}} }

pub trait NthOne<T> {
    fn nth_one(&self, n: usize) -> Result<usize, ()> where Self: Sized;
}
impl NthOne<u8> for u8 { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..8 {if self & (1_u8 << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }
impl NthOne<u16> for u16 { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..16 {if self & (1_u16 << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }
impl NthOne<u32> for u32 { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..32 {if self & (1_u32 << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }
impl NthOne<u64> for u64 { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..64 {if self & (1_u64 << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }
impl NthOne<u128> for u128 { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..128 {if self & (1_u128 << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }
impl NthOne<usize> for usize { fn nth_one(&self, n: usize) -> Result<usize, ()> {if n < self.count_ones() as usize {let mut count = 0_usize; for i in 0..(8 * mem::size_of::<usize>()) {if self & (1_usize << i) != 0 {if count == n {return Ok(i);} count += 1;}} Err(())} else {Err(())}} }

struct BitIndexedArrayImpl <B, V, const SIZE: usize> {
    bits: B,
    values: [V; SIZE],
}

impl<B: CountOnes<B>, V, const SIZE: usize> BitIndexedArrayImpl<B, V, SIZE> {
    fn new(bits: B, values: impl Into<VecDeque<V>>) -> Result<Self, ()> {
        let mut values: VecDeque<V> = values.into();
        if bits.count_ones_t() != SIZE || values.len() != SIZE {
            return Err(());
        }
        let values = unsafe {
            #[allow(deprecated)]
            let mut building: [V; SIZE] = mem::uninitialized();
            for dest in building.iter_mut() {
                ptr::write(dest, values.pop_front().unwrap());
            }
            building
        };
        Ok(Self { bits, values })
    }
}

pub trait BitIndexedArray<B, V: Clone> {
    fn bits(&self) -> B;
    fn len(&self) -> usize;

    fn at(&self, bit: B) -> Result<&V, ()>;
    fn at_index(&self, index: usize) -> Result<&V, ()>;
    fn inserted(&self, bit: B, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;
    fn inserted_index(&self, index: usize, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;
    fn updated(&self, bit: B, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;
    fn updated_index(&self, index: usize, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;
    fn removed(&self, bit: B) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;
    fn removed_index(&self, index: usize) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()>;

    fn clone_impl(&self) -> Box<dyn BitIndexedArray::<B, V>>;
    fn eq_impl(&self, other: &dyn BitIndexedArray::<B, V>) -> bool;
    fn fmt_impl(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error>;
    
    fn iter(&'_ self) -> core::slice::Iter<'_, V>;
    fn iter_mut(&'_ mut self) -> core::slice::IterMut<'_, V>;
}
macro_rules! bit_indexed_array_t {
    ( $size:literal ) => {
        impl <B: BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + NthBit<B> + NthOne<B> + PartialEq + 'static, V: Clone + Debug + PartialEq + 'static> BitIndexedArray<B, V> for BitIndexedArrayImpl<B, V, $size> {
            fn bits(&self) -> B {
                self.bits.clone()
            }

            fn len(&self) -> usize {
                $size
            }

            fn at(&self, bit: B) -> Result<&V, ()> {
                let index = self.bits.bit_index(bit)?;
                #[allow(unused_comparisons)]
                if index < $size {
                    Ok(&self.values[index])
                }
                else {
                    Err(())
                }
            }

            fn at_index(&self, index: usize) -> Result<&V, ()> {
                Ok(&self.values[self.bits.bit_index(B::nth_bit(index)?)?])
            }

            fn inserted(&self, bit: B, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                let bits = self.bits.bit_insert(bit.clone())?;
                let index = bits.bit_index(bit)?;
                let mut building = VecDeque::<V>::new();
                for i in 0..index {
                    building.push_back(self.values[i].clone());
                }
                building.push_back(value.into_owned());
                for i in index..$size {
                    building.push_back(self.values[i].clone());
                }
                new_bit_indexed_array(bits, building)
            }

            fn inserted_index(&self, index: usize, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                self.inserted(B::nth_bit(index)?, value)
            }
            
            fn updated(&self, bit: B, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                if !self.bits.bit_contains(bit.clone())? {
                    return Err(());
                }
                let index = self.bits.bit_index(bit)?;
                let mut building = VecDeque::<V>::new();
                for i in 0..index {
                    building.push_back(self.values[i].clone());
                }
                building.push_back(value.into_owned());
                for i in index+1..$size {
                    building.push_back(self.values[i].clone());
                }
                new_bit_indexed_array(self.bits.clone(), building)
            }

            fn updated_index(&self, index: usize, value: Cow<V>) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                self.updated(B::nth_bit(index)?, value)
            }
            
            fn removed(&self, bit: B) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                let bits = self.bits.bit_remove(bit.clone())?;
                let index = self.bits.bit_index(bit)?;
                let mut building = VecDeque::<V>::new();
                for i in 0..index {
                    building.push_back(self.values[i].clone());
                }
                for i in index+1..$size {
                    building.push_back(self.values[i].clone());
                }
                new_bit_indexed_array(bits, building)
            }
            
            fn removed_index(&self, index: usize) -> Result<Box<dyn BitIndexedArray::<B, V>>, ()> {
                self.removed(B::nth_bit(index)?)
            }
            
            fn clone_impl(&self) -> Box<dyn BitIndexedArray::<B, V>> {
                Box::new(Self {
                    bits: self.bits.clone(),
                    values: self.values.clone(),
                })
            }
            
            fn eq_impl(&self, other: &dyn BitIndexedArray::<B, V>) -> bool {
                if self.len() != other.len() {
                    return false;
                }
                let other = unsafe { &*(other as *const dyn BitIndexedArray::<B, V> as *const Self) };
                self.bits == other.bits && self.values == other.values
            }
            
            fn fmt_impl(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
                write!(f, "BitIndexedArrayImpl {{ bits: {:?}, values: {:?} }}", self.bits, self.values)
            }
            
            fn iter<'a>(&'a self) -> core::slice::Iter<'a, V> {
                self.values.iter()
            }
            
            fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, V> {
                self.values.iter_mut()
            }
        }
    };
    ( $size:literal, $($rest:literal),+ ) => {
        bit_indexed_array_t!($size);
        bit_indexed_array_t!($($rest),+);
    };
}
bit_indexed_array_t!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
    64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
    96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128);

#[allow(dead_code)]
pub fn default_bit_indexed_array<B: BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + Default + NthBit<B> + NthOne<B> + PartialEq + 'static, V: Clone + Debug + PartialEq + 'static>() -> Box<dyn BitIndexedArray<B, V>> {
    Box::new(BitIndexedArrayImpl::<B, V, 0>::default())
}

#[allow(dead_code)]
pub fn new_bit_indexed_array<B: BitContains<B> + BitIndex<B> + BitInsert<B> + BitRemove<B> + Clone + CountOnes<B> + Debug + NthBit<B> + NthOne<B> + PartialEq + 'static, V: Clone + Debug + PartialEq + 'static>(bits: B, values: impl Into<VecDeque<V>>) -> Result<Box<dyn BitIndexedArray<B, V>>, ()> {
    let values: VecDeque<V> = values.into();
    if bits.count_ones_t() != values.len() {
        return Err(());
    }
    match values.len() {
        0 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 0>::new(bits, values).unwrap())),
        1 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 1>::new(bits, values).unwrap())),
        2 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 2>::new(bits, values).unwrap())),
        3 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 3>::new(bits, values).unwrap())),
        4 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 4>::new(bits, values).unwrap())),
        5 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 5>::new(bits, values).unwrap())),
        6 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 6>::new(bits, values).unwrap())),
        7 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 7>::new(bits, values).unwrap())),
        8 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 8>::new(bits, values).unwrap())),
        9 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 9>::new(bits, values).unwrap())),
        10 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 10>::new(bits, values).unwrap())),
        11 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 11>::new(bits, values).unwrap())),
        12 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 12>::new(bits, values).unwrap())),
        13 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 13>::new(bits, values).unwrap())),
        14 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 14>::new(bits, values).unwrap())),
        15 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 15>::new(bits, values).unwrap())),
        16 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 16>::new(bits, values).unwrap())),
        17 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 17>::new(bits, values).unwrap())),
        18 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 18>::new(bits, values).unwrap())),
        19 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 19>::new(bits, values).unwrap())),
        20 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 20>::new(bits, values).unwrap())),
        21 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 21>::new(bits, values).unwrap())),
        22 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 22>::new(bits, values).unwrap())),
        23 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 23>::new(bits, values).unwrap())),
        24 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 24>::new(bits, values).unwrap())),
        25 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 25>::new(bits, values).unwrap())),
        26 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 26>::new(bits, values).unwrap())),
        27 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 27>::new(bits, values).unwrap())),
        28 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 28>::new(bits, values).unwrap())),
        29 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 29>::new(bits, values).unwrap())),
        30 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 30>::new(bits, values).unwrap())),
        31 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 31>::new(bits, values).unwrap())),
        32 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 32>::new(bits, values).unwrap())),
        33 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 33>::new(bits, values).unwrap())),
        34 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 34>::new(bits, values).unwrap())),
        35 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 35>::new(bits, values).unwrap())),
        36 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 36>::new(bits, values).unwrap())),
        37 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 37>::new(bits, values).unwrap())),
        38 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 38>::new(bits, values).unwrap())),
        39 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 39>::new(bits, values).unwrap())),
        40 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 40>::new(bits, values).unwrap())),
        41 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 41>::new(bits, values).unwrap())),
        42 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 42>::new(bits, values).unwrap())),
        43 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 43>::new(bits, values).unwrap())),
        44 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 44>::new(bits, values).unwrap())),
        45 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 45>::new(bits, values).unwrap())),
        46 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 46>::new(bits, values).unwrap())),
        47 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 47>::new(bits, values).unwrap())),
        48 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 48>::new(bits, values).unwrap())),
        49 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 49>::new(bits, values).unwrap())),
        50 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 50>::new(bits, values).unwrap())),
        51 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 51>::new(bits, values).unwrap())),
        52 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 52>::new(bits, values).unwrap())),
        53 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 53>::new(bits, values).unwrap())),
        54 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 54>::new(bits, values).unwrap())),
        55 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 55>::new(bits, values).unwrap())),
        56 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 56>::new(bits, values).unwrap())),
        57 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 57>::new(bits, values).unwrap())),
        58 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 58>::new(bits, values).unwrap())),
        59 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 59>::new(bits, values).unwrap())),
        60 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 60>::new(bits, values).unwrap())),
        61 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 61>::new(bits, values).unwrap())),
        62 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 62>::new(bits, values).unwrap())),
        63 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 63>::new(bits, values).unwrap())),
        64 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 64>::new(bits, values).unwrap())),
        65 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 65>::new(bits, values).unwrap())),
        66 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 66>::new(bits, values).unwrap())),
        67 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 67>::new(bits, values).unwrap())),
        68 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 68>::new(bits, values).unwrap())),
        69 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 69>::new(bits, values).unwrap())),
        70 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 70>::new(bits, values).unwrap())),
        71 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 71>::new(bits, values).unwrap())),
        72 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 72>::new(bits, values).unwrap())),
        73 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 73>::new(bits, values).unwrap())),
        74 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 74>::new(bits, values).unwrap())),
        75 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 75>::new(bits, values).unwrap())),
        76 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 76>::new(bits, values).unwrap())),
        77 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 77>::new(bits, values).unwrap())),
        78 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 78>::new(bits, values).unwrap())),
        79 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 79>::new(bits, values).unwrap())),
        80 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 80>::new(bits, values).unwrap())),
        81 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 81>::new(bits, values).unwrap())),
        82 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 82>::new(bits, values).unwrap())),
        83 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 83>::new(bits, values).unwrap())),
        84 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 84>::new(bits, values).unwrap())),
        85 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 85>::new(bits, values).unwrap())),
        86 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 86>::new(bits, values).unwrap())),
        87 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 87>::new(bits, values).unwrap())),
        88 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 88>::new(bits, values).unwrap())),
        89 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 89>::new(bits, values).unwrap())),
        90 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 90>::new(bits, values).unwrap())),
        91 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 91>::new(bits, values).unwrap())),
        92 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 92>::new(bits, values).unwrap())),
        93 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 93>::new(bits, values).unwrap())),
        94 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 94>::new(bits, values).unwrap())),
        95 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 95>::new(bits, values).unwrap())),
        96 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 96>::new(bits, values).unwrap())),
        97 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 97>::new(bits, values).unwrap())),
        98 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 98>::new(bits, values).unwrap())),
        99 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 99>::new(bits, values).unwrap())),
        100 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 100>::new(bits, values).unwrap())),
        101 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 101>::new(bits, values).unwrap())),
        102 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 102>::new(bits, values).unwrap())),
        103 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 103>::new(bits, values).unwrap())),
        104 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 104>::new(bits, values).unwrap())),
        105 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 105>::new(bits, values).unwrap())),
        106 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 106>::new(bits, values).unwrap())),
        107 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 107>::new(bits, values).unwrap())),
        108 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 108>::new(bits, values).unwrap())),
        109 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 109>::new(bits, values).unwrap())),
        110 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 110>::new(bits, values).unwrap())),
        111 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 111>::new(bits, values).unwrap())),
        112 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 112>::new(bits, values).unwrap())),
        113 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 113>::new(bits, values).unwrap())),
        114 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 114>::new(bits, values).unwrap())),
        115 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 115>::new(bits, values).unwrap())),
        116 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 116>::new(bits, values).unwrap())),
        117 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 117>::new(bits, values).unwrap())),
        118 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 118>::new(bits, values).unwrap())),
        119 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 119>::new(bits, values).unwrap())),
        120 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 120>::new(bits, values).unwrap())),
        121 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 121>::new(bits, values).unwrap())),
        122 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 122>::new(bits, values).unwrap())),
        123 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 123>::new(bits, values).unwrap())),
        124 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 124>::new(bits, values).unwrap())),
        125 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 125>::new(bits, values).unwrap())),
        126 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 126>::new(bits, values).unwrap())),
        127 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 127>::new(bits, values).unwrap())),
        128 => Ok(Box::new(BitIndexedArrayImpl::<B, V, 128>::new(bits, values).unwrap())),
        _ => Err(())
    }
}

impl <B: Clone, V: Clone, const SIZE: usize> Clone for BitIndexedArrayImpl<B, V, SIZE> {
    fn clone(&self) -> Self {
        Self {
            bits: self.bits.clone(),
            values: self.values.clone(),
        }
    }
}

impl <B: Clone, V: Clone> Clone for Box<dyn BitIndexedArray<B, V>> {
    fn clone(&self) -> Self {
        self.clone_impl()
    }
}

impl <B: CountOnes<B> + Default, V> Default for BitIndexedArrayImpl<B, V, 0> {
    fn default() -> Self {
        Self::new(B::default(), Vec::new()).unwrap()
    }
}

impl <B: Eq, V: Eq, const SIZE: usize> Eq for BitIndexedArrayImpl<B, V, SIZE> {}

impl <B: Eq, V: Clone + Eq> Eq for dyn BitIndexedArray<B, V> {}

impl <B: PartialEq, V: PartialEq, const SIZE: usize> PartialEq for BitIndexedArrayImpl<B, V, SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits && self.values == other.values
    }
}

impl <B: PartialEq, V: Clone + PartialEq> PartialEq for dyn BitIndexedArray<B, V> {
    fn eq(&self, other: &Self) -> bool {
        self.eq_impl(other)
    }
}

impl <'a, B, V, const SIZE: usize> IntoIterator for &'a BitIndexedArrayImpl<B, V, SIZE> {
    type Item = &'a V;
    type IntoIter = core::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl <'a, B, V, const SIZE: usize> IntoIterator for &'a mut BitIndexedArrayImpl<B, V, SIZE> {
    type Item = &'a mut V;
    type IntoIter = core::slice::IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl <'a, B, V: Clone> IntoIterator for &'a dyn BitIndexedArray<B, V> {
    type Item = &'a V;
    type IntoIter = core::slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl <'a, B, V: Clone> IntoIterator for &'a mut dyn BitIndexedArray<B, V> {
    type Item = &'a mut V;
    type IntoIter = core::slice::IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl <B: Debug, V: Debug, const SIZE: usize> Debug for BitIndexedArrayImpl<B, V, SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "BitIndexedArrayImpl {{ bits: {:?}, values: {:?} }}", self.bits, self.values)
    }
}

impl <B: Debug, V: Clone + Debug> Debug for dyn BitIndexedArray<B, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        self.fmt_impl(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn bit_indexed_array_insert() {
        let mut bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        bia = bia.as_ref().inserted(0b10, Cow::Owned(3)).unwrap();
        assert_eq!(bia.as_ref().len(), 3);
        assert_eq!(*bia.as_ref().at_index(0).unwrap(), 13);
        assert_eq!(*bia.as_ref().at_index(1).unwrap(), 3);
        assert_eq!(*bia.as_ref().at_index(2).unwrap(), 42);
        assert!(bia.as_ref().at_index(3).is_err());
        assert_eq!(*bia.as_ref().at(0b1).unwrap(), 13);
        assert_eq!(*bia.as_ref().at(0b10).unwrap(), 3);
        assert_eq!(*bia.as_ref().at(0b100).unwrap(), 42);
        assert!(bia.as_ref().at(0b1000).is_err());
        println!("Array: {:?}", bia.as_ref());
    }

    #[test]
    fn bit_indexed_array_insert_reinsert_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        assert!(bia.as_ref().inserted(0b100, Cow::Owned(3)).is_err());
    }

    #[test]
    fn bit_indexed_array_insert_multibit_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        assert!(bia.as_ref().inserted(0b1010, Cow::Owned(3)).is_err());
    }

    #[test]
    fn bit_indexed_array_update() {
        let mut bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,3>::new(0b1101_u64, vec!(13, 42, 8)).unwrap());
        bia = bia.as_ref().updated(0b1000, Cow::Owned(11)).unwrap();
        assert_eq!(bia.as_ref().len(), 3);
        assert_eq!(*bia.as_ref().at_index(0).unwrap(), 13);
        assert!(bia.as_ref().at_index(1).is_err());
        assert_eq!(*bia.as_ref().at_index(2).unwrap(), 42);
        assert_eq!(*bia.as_ref().at_index(3).unwrap(), 11);
        assert!(bia.as_ref().at_index(4).is_err());
        assert_eq!(*bia.as_ref().at(0b1).unwrap(), 13);
        assert!(bia.as_ref().at(0b10).is_err());
        assert_eq!(*bia.as_ref().at(0b100).unwrap(), 42);
        assert_eq!(*bia.as_ref().at(0b1000).unwrap(), 11);
        assert!(bia.as_ref().at(0b10000).is_err());
        println!("Array: {:?}", bia.as_ref());
    }

    #[test]
    fn bit_indexed_array_update_absent_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        assert!(bia.as_ref().updated(0b10, Cow::Owned(3)).is_err());
    }

    #[test]
    fn bit_indexed_array_update_multibit_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        assert!(bia.as_ref().updated(0b101, Cow::Owned(3)).is_err());
    }

    #[test]
    fn bit_indexed_array_update_index() {
        let mut bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,3>::new(0b1101_u64, vec!(13, 42, 8)).unwrap());
        bia = bia.as_ref().updated_index(2, Cow::Owned(11)).unwrap();
        assert_eq!(bia.as_ref().len(), 3);
        assert_eq!(*bia.as_ref().at_index(0).unwrap(), 13);
        assert!(bia.as_ref().at_index(1).is_err());
        assert_eq!(*bia.as_ref().at_index(2).unwrap(), 11);
        assert_eq!(*bia.as_ref().at_index(3).unwrap(), 8);
        assert!(bia.as_ref().at_index(4).is_err());
        assert_eq!(*bia.as_ref().at(0b1).unwrap(), 13);
        assert!(bia.as_ref().at(0b10).is_err());
        assert_eq!(*bia.as_ref().at(0b100).unwrap(), 11);
        assert_eq!(*bia.as_ref().at(0b1000).unwrap(), 8);
        assert!(bia.as_ref().at(0b10000).is_err());
        println!("Array: {:?}", bia.as_ref());
    }

    #[test]
    fn bit_indexed_array_update_absent_index_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,2>::new(0b101_u64, vec!(13, 42)).unwrap());
        assert!(bia.as_ref().updated_index(1, Cow::Owned(3)).is_err());
    }

    #[test]
    fn bit_indexed_array_remove() {
        let mut bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,3>::new(0b1101_u64, vec!(13, 42, 8)).unwrap());
        bia = bia.as_ref().removed(0b1000).unwrap();
        assert_eq!(bia.as_ref().len(), 2);
        assert_eq!(*bia.as_ref().at_index(0).unwrap(), 13);
        assert!(bia.as_ref().at_index(1).is_err());
        assert_eq!(*bia.as_ref().at_index(2).unwrap(), 42);
        assert!(bia.as_ref().at_index(3).is_err());
        assert!(bia.as_ref().at_index(4).is_err());
        assert_eq!(*bia.as_ref().at(0b1).unwrap(), 13);
        assert!(bia.as_ref().at(0b10).is_err());
        assert_eq!(*bia.as_ref().at(0b100).unwrap(), 42);
        assert!(bia.as_ref().at(0b1000).is_err());
        println!("Array: {:?}", bia.as_ref());
    }

    #[test]
    fn bit_indexed_array_remove_absent_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,3>::new(0b1101_u64, vec!(13, 42, 8)).unwrap());
        assert!(bia.as_ref().removed(0b10).is_err());
    }

    #[test]
    fn bit_indexed_array_remove_multibit_failure() {
        let bia: Box<dyn BitIndexedArray<_,_>> = Box::new(BitIndexedArrayImpl::<_,_,3>::new(0b1101_u64, vec!(13, 42, 8)).unwrap());
        assert!(bia.as_ref().removed(0b101).is_err());
    }

}
