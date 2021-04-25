mod cnode;
mod lnode;
mod mnode;
mod snode;

use alloc::sync::Arc;
use core::{hash::Hash, ptr};
use mnode::MNode;

#[derive(Debug)]
pub struct HashTrie <T: Clone + Eq + PartialEq + Hash> {
    root: Option<Arc<dyn MNode<T>>>,
}

impl <T: Clone + Eq + PartialEq + Hash> HashTrie<T> {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn find(&self, value: T) -> Option<&T> {
        None
    }

    fn insert(&self, value: T) -> Self {
        self.clone()
    }

    fn remove(&self, value: T) -> Self {
        self.clone()
    }
}

impl <T: Clone + Eq + PartialEq + Hash> Clone for HashTrie<T> {
    fn clone(&self) -> Self {
        Self {
            root: match &self.root {
                Some(mnode) => Some(mnode.clone()),
                None => None
            }
        }
    }
}

impl <T: Clone + Eq + PartialEq + Hash> Default for HashTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl <T: Clone + Eq + PartialEq + Hash> Eq for HashTrie<T> {}

impl <T: Clone + Eq + PartialEq + Hash> PartialEq<HashTrie<T>> for HashTrie<T> {
    fn eq(&self, other: &Self) -> bool {
        match &self.root {
            Some(left) => match &other.root {
                Some(right) => ptr::eq(Arc::as_ptr(left) as *const u8, Arc::as_ptr(right) as *const u8),
                None => false
            },
            None => other.root.is_none()
        }
    }
}
