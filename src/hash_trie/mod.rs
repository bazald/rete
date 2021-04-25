mod cnode;
mod lnode;
mod mnode;
mod snode;

use alloc::{borrow::Cow, fmt::Debug, sync::Arc};
use core::hash::Hash;
use mnode::*;
use snode::SNode;

#[derive(Debug, Eq, PartialEq)]
pub struct HashTrie <T: Clone + Debug + Eq + PartialEq + Hash + 'static> {
    root: Option<Arc<dyn MNode<T>>>,
}

impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> HashTrie<T> {
    fn new() -> Self {
        Self {
            root: None
        }
    }

    fn singleton(mnode: Arc<dyn MNode<T>>) -> Self {
        Self {
            root: Some(mnode)
        }
    }

    #[allow(dead_code)]
    fn find(&self, value: &T) -> Option<&T> {
        match &self.root {
            Some(root) => match root.find(value) {
                FindResult::NotFound => None,
                FindResult::Found(found) => Some(found)
            },
            None => None
        }
    }

    #[allow(dead_code)]
    fn insert<'a>(&'a self, value: Cow<T>) -> (Self, &'a T) {
        match &self.root {
            Some(root) => match root.insert(value) {
                InsertResult::Found(found) => (Self::singleton(root.clone()), found),
                InsertResult::Inserted(mnode, inserted) => (Self::singleton(mnode), inserted)
            },
            None => {
                let created = Self::singleton(SNode::new(value.into_owned()));
                let snode = unsafe { (Arc::as_ptr(created.root.as_ref().unwrap()) as *const SNode<T>).as_ref().unwrap() };
                (created, snode.get())
            }
        }
    }

    #[allow(dead_code)]
    fn remove(&self, value: &T) -> Self {
        self.clone() // TOOD: actually do removal
    }
}

impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> Clone for HashTrie<T> {
    fn clone(&self) -> Self {
        Self {
            root: match &self.root {
                Some(mnode) => Some(mnode.clone()),
                None => None
            }
        }
    }
}

impl <T: Clone + Debug + Eq + PartialEq + Hash + 'static> Default for HashTrie<T> {
    fn default() -> Self {
        Self::new()
    }
}
